- Feature Name: extending-types
- Start Date: 2022-07-11
- RFC PR: [blockprotocol/blockprotocol#428](https://github.com/blockprotocol/blockprotocol/pull/428)
- Block Protocol Discussion: [blockprotocol/blockprotocol#439](https://github.com/blockprotocol/blockprotocol/discussions/439)

# Summary

[summary]: #summary

As a follow-up to the [Graph type system RFC](./0352-graph-type-system.md), this RFC will describe the behavior of which types in the type system can be extended to enhance reusability while giving more control to users (both block authors and users of embedding applications). Extending types can be considered the same as "type inheritance", but there are some important nuances that make these concepts different.

This RFC is not set out to solve duplicating types or "forking", and this will be considered an unresolved problem in this RFC.

# Motivation

[motivation]: #motivation

Reusing public types in the type system comes with the potential disadvantage of not fully conforming to a user's intention. If a user is interested in a public type but needs certain additional properties to make the type usable for their use case, they would have to recreate the type themself in the current system.

Allowing for types to be extended in the Block Protocol means that a user could still make use of public types when they want to define types for their domain.

This RFC introduces a way for types to be extended in a way where the reusability and sharing aspects of the Block Protocol are maintained.

# Guide-level explanation

[guide-level-explanation]: #guide-level-explanation

Type extension can be seen as the concept of adding properties to an existing entity type `Type` by creating a new type `SubType` that has a specific relation to `Type`.
Using `SubType` in place of `Type` must be possible when extending a type, which means that existing properties and links may _not_ be modified.

For example, an `Employee` entity type could be an extended version of `Person`. This `Employee` type could contain all of the properties of `Person` as well as having _additional_ domain-specific properties, making it more concrete while keeping compatibility with `Person`.

If the `Person` entity type contains required properties `name` and `age`, the `Employee` entity type would inherit these properties and could add other properties (e.g. an `occupation` property). `Employee` would _not_ be able to override any of the properties inherited from `Person` (e.g. it's not possible to turn `name` into an array or make it optional) to ensure that instances of `Employee` are also valid instances of `Person` (i.e. compatibility with `Person` is preserved).

As a consequence of the above definition, these questions arise:

- How do we ensure that `Employee` instances can, in fact, be used in place of `Person` instances in practice?
- Do multiple supertypes impose constraints on extending types?
- How does having `additionalProperties` in existing schemas influence extended types?
- How should the Block Protocol type system support extending types?

## Subtyping

Compatibility between extended types is the ability to use the subtype in place of a supertype. [Prior art](https://en.wikipedia.org/wiki/Subtyping#Coercions) calls this `coercive subtyping` when it can happen implicitly.
_Composition_ rather than inheritance allows us to have more guarantees about the relationship between subtypes and supertypes. If a subtype never modifies a supertype's inherited properties, we are sure that the supertype properties are left untouched, which eliminates the need for evaluating compatibility (i.e. no need for [`subsumption`](https://en.wikipedia.org/wiki/Subtyping#Subsumption), the concept of finding out whether or not a supertype is a supertype of a subtype or not).

For example, an `Employee` instance looks as follows (simplified):

```json
{
  "name": "Charles",
  "age": 35,
  "occupation": "Merchant"
}
```

And the properties would have the following relation to `Employee`:

```txt
      name◄┐ (supertype)    (subtype)
           ├───Person───────Employee
       age◄┘                │
                            │
occupation◄─────────────────┘
```

We can visually see how selecting `Person` in the type hierarchy would provide `name` and `age` properties but exclude the `occupation` property.
Assuming that we are able to project/select the properties of a type that are defined through the supertype, coercive subtyping is attainable for any subtype. This is a somewhat strong assumption to make, but it unlocks expressing how to extend types.

## Multiple supertypes

A type must allow extending multiple supertypes if and only if the supertypes can coexist. For supertypes to be able to coexist, their properties should either be disjoint, or overlap in a compatible manner.

**An example of _disjoint_ properties**:

- Supertype `Person` contains required properties `name` and `age`
- Supertype `Superhero` contains the property `superpower`

In this example, there is no overlap between properties, so an `Employee` type could have `Person` and `Superhero` as supertypes

```txt
              (supertypes)
superpower◄────Superhero──┐
                          │
      name◄┐              │
           ├───Person─────Employee (subtype)
       age◄┘              │
                          │
occupation◄───────────────┘
```

**An example of _compatible_, overlapping properties**:

- Supertype `Person` contains the required properties `name` and `age`
- Supertype `Superhero` contains the required properties `superpower` and `name`

In this example, `name` overlaps as a required property in both supertypes.

```txt
              (supertypes)
superpower◄────Superhero──┐
               │          │
      name◄────┤          │
               │          │
       age◄────Person─────Employee (subtype)
                          │
occupation◄───────────────┘


```

**An example of _incompatible_, overlapping properties**:

- Supertype `Person` contains the required properties `name` and `age`
- Supertype `Superhero` contains the required property `superpower` and an array of `name`s

In this example, the array of `name`s on the `Superhero` type would not be compatible with the required `name` property of `Person`, which means that the two types cannot be supertypes together.

## Additional properties on types

In the proposed [Versioning RFC](./0408-versioning-types.md) for the type system, having `{ "additionalProperties": false }` for all schemas is an assumption made for determining type compatibility, which means that any supertype will not validate against a subtype that adds properties if it receives all properties of a subtype instance. For example, if we supply the `Employee` instance from above to a `Person`, it will receive properties that are considered `additionalProperties` (the `occupation` property is not present on `Person`).

The assumption that we can select/project parts of a subtype that make up a supertype is essential for keeping strictness in JSON Schemas.

We propose slight modifications to how `{ "additionalProperties": true }` and `{ "unevaluatedProperties": false }` behave and may be used within our type extension system to make supertypes keep strictness while allowing composition.

Concrete examples of how JSON Schema breaks with these validation constraints are shown in the [Reference-level explanation](#additional-properties-on-types-1)

## Defining extended types

Extended types will be defined with conventional JSON Schema syntax, the `allOf` keyword. An entity type can extend another entity type by adding an entry to `allOf` value with a versioned URI reference.
Using a [versioned URI](https://github.com/blockprotocol/blockprotocol/blob/main/rfcs/text/0408-versioning-types.md#type-uris) makes it so that subtypes aren't automatically updated when the supertype is.

As extended types can extend other extended types, we must also make sure that there are no cycles within the type hierarchy, as it makes types difficult to resolve/reason about, and could lead to unpredictable behaviour.

## Addressing previous considerations

In the [Type System RFC](./0352-graph-type-system.md#interfacing-with-types-1) the following statement was made:

> The update messages of both the current and new systems make use of partial schemas, merging the schema given in the message contents with the existing Entity Type. This may or may not be the desired semantics of updating, and could lead to undesired behavior. In that case, the semantics can be changed to treat updates as a complete replacement. This is to be decided and can be considered out of scope for this RFC as it touches on inheritance/forking concepts.

As the new Block Protocol type system doesn't require a delta-based storage approach for its schemas, it's unclear what the advantages of partial schemas for update requests are. We propose that we should treat type updates as complete replacements, so implementation is much more straightforward for embedding applications. Partial schema updates also add some level of indirection, and that may obfuscate error sources and error reasons. Therefore, updates to types must be complete replacements rather than partial schema updates.

# Reference-level explanation

[reference-level-explanation]: #reference-level-explanation

## Defining extended types

In the Block Protocol, we will allow type extension through the `allOf` JSON Schema keyword which specifies an array of schemas that will have to validate together.

We'll add the following fields to the existing Entity Type meta schema definition:

```json
{
  "$id": "https://blockprotocol.org/type-system/0.2/schema/meta/entity-type",
  "type": "object",
  ...,
  "properties": {
    ...,
    "allOf": {
      "type": "array",
      "items": {
      "type": "object",
      "properties": {
        "$ref": {
          "$comment": "Valid reference to an existing Entity Type version",
          "type": "string",
          "format": "uri"
        },
        "additionalProperties": false
      }
    }
    }

  }
}
```

### Concrete examples of extended types

**An example of _disjoint_ properties**:

Given a _supertype_ `Person`:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/person/v/1",
  "type": "object",
  "title": "Person",
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/age": {
      "$ref": "https://blockprotocol.org/@alice/property-type/age/v/1"
    }
  },
  "required": [
    "https://blockprotocol.org/@alice/property-type/name",
    "https://blockprotocol.org/@alice/property-type/age"
  ]
}
```

and a _subtype_ `Employee`:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/employee/v/1",
  "type": "object",
  "title": "Employee",
  "allOf": [
    { "$ref": "https://blockprotocol.org/@alice/entity-type/person/v/1" }
  ],
  "properties": {
    "https://blockprotocol.org/@alice/property-type/occupation": {
      "$ref": "https://blockprotocol.org/@alice/property-type/occupation/v/1"
    }
  },
  "required": ["https://blockprotocol.org/@alice/property-type/occupation"]
}
```

The two types do not share any properties, which establishes compatibility. It is possible to coerce an instance of `Employee` into an instance of `Person` as the properties compose and are compatible.
For example, we may have the following `Employee` instance:

```json
{
  "entityId": 111,
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": "Charles",
    "https://blockprotocol.org/@alice/property-type/age": 35,
    "https://blockprotocol.org/@alice/property-type/occupation": "Merchant"
  }
}
```

which can be coerced into the following `Person` instance:

```json
{
  "entityId": 111,
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": "Charles",
    "https://blockprotocol.org/@alice/property-type/age": 35
  }
}
```

Notice how we are keeping the same `entityId` for this entity instance, but simply coercing the entity instance by selecting the properties of interest for the given type.

**An example of _compatible_, overlapping properties**:

Given a _supertype_ `Person`:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/person/v/2",
  "type": "object",
  "title": "Person",
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/age": {
      "$ref": "https://blockprotocol.org/@alice/property-type/age/v/1"
    }
  },
  "required": [
    "https://blockprotocol.org/@alice/property-type/name",
    "https://blockprotocol.org/@alice/property-type/age"
  ]
}
```

and a _subtype_ `Employee`:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/employee/v/2",
  "type": "object",
  "title": "Employee",
  "allOf": [
    { "$ref": "https://blockprotocol.org/@alice/entity-type/person/v/2" }
  ],
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/occupation": {
      "$ref": "https://blockprotocol.org/@alice/property-type/occupation/v/1"
    }
  },
  "required": [
    "https://blockprotocol.org/@alice/property-type/name",
    "https://blockprotocol.org/@alice/property-type/occupation"
  ]
}
```

The two types do share the `name` property, but the property definitions are compatible (even using the same version). It's possible to coerce an instance of `Employee` into an instance of `Person` as the properties compose and are compatible. Checking compatibility is described in the [Reference-level explanation](#multiple-supertypes---checking-compatibility).
For example, we may have the following `Employee` instance:

```json
{
  "entityId": 112,
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": "Charles",
    "https://blockprotocol.org/@alice/property-type/age": 35,
    "https://blockprotocol.org/@alice/property-type/occupation": "Merchant"
  }
}
```

which can be coerced into the following `Person` instance:

```json
{
  "entityId": 112,
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": "Charles",
    "https://blockprotocol.org/@alice/property-type/age": 35
  }
}
```

## Subtyping

We're already implicitly defining `{ "additionalProperties": false }` in the [versioning RFC](./0408-versioning-types.md#determining-type-compatibility) for all schemas, this RFC will piggyback on that, and change the existing, implicit `additionalProperties` usage to `unevaluatedProperties` and slightly change its [vocabulary definition](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties).

The vocabulary change is that the `unevaluatedProperties` keyword will only be applicable at top-level schemas (i.e. subtypes or schemas that don't specify `allOf`), and ignored when present on a schema that resides in the `allOf` array. Instead of the default value being `{ "unevaluatedProperties": {} }`, the default value will be `{ "unevaluatedProperties": false }`. Together this makes it so that all supertypes will be able to compose, as their individual, implicit `unevaluatedProperties` values won't have any effect on validation and defer evaluation to the extending type (the subtype).

Concretely, this means that for a free-standing type that doesn't extend any other type such as this `Person` type:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/person/v/2",
  "type": "object",
  "title": "Person",
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/age": {
      "$ref": "https://blockprotocol.org/@alice/property-type/age/v/1"
    }
  },
  "required": [
    "https://blockprotocol.org/@alice/property-type/name",
    "https://blockprotocol.org/@alice/property-type/age"
  ]
}
```

would implicitly have `{ "unevaluatedProperties": false }` set. In the case of the `Employee` type:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/employee/v/2",
  "type": "object",
  "title": "Employee",
  "allOf": [
    { "$ref": "https://blockprotocol.org/@alice/entity-type/person/v/2" }
  ],
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/occupation": {
      "$ref": "https://blockprotocol.org/@alice/property-type/occupation/v/1"
    }
  },
  "required": [
    "https://blockprotocol.org/@alice/property-type/name",
    "https://blockprotocol.org/@alice/property-type/occupation"
  ]
}
```

the resolved schema residing at `{ "$ref": "https://blockprotocol.org/@alice/entity-type/person/v/2" }` would _not_ have `{ "unevaluatedProperties": false }` set, whereas the top-level entity type itself would have `{ "unevaluatedProperties": false }` set.

### Liskov substitution principle

Subtypes in the system can be used in place of the supertypes that they extend, similarly to what is expected in the Liskov substitution principle. Although our entity types do not contain behavior, the semantic meaning of individual properties on an entity type is captured on the property type definitions they refer to, which enables the ability to use the subtype in place of its supertype(s) "without altering any of the desirable properties of that program" [[source](https://en.wikipedia.org/wiki/Liskov_substitution_principle#Principle)].

## Detecting cycles

> 💭 Because of versioning mechanics in the type system added in the [versioning RFC](./0408-versioning-types.md), type versions are not able to make out proper dependency cyles, and would not allow cyclic type hierachy to exist in the literal sense as updates to types would always result in a new version identifier. Instead cycles in the below explanation is more about indirection and obfuscation of types.

An extension cycle happens when a part of an inheritance tree revisits a base URI it has already seen. As a contrived example, an entity type `Country` could be the supertype of `Region`, which in turn could be a supertype of the same `Country` entity type.

First entity type version of `Country` without supertype:

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/country/v/1",
  "type": "object",
  "title": "Country",
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    }
  },
  "required": ["https://blockprotocol.org/@alice/property-type/name"]
}
```

Entity type `Region` with `Country` as a supertype

```json
{
  "kind": "entityType",
  "$id": "https://blockprotocol.org/@alice/entity-type/region/v/1",
  "type": "object",
  "title": "Region",
  "allOf": [
    { "$ref": "https://blockprotocol.org/@alice/entity-type/country/v/1" }
  ],
  "properties": {
    "https://blockprotocol.org/@alice/property-type/blurb": {
      "$ref": "https://blockprotocol.org/@alice/property-type/blurb/v/1"
    }
  },
  "required": ["https://blockprotocol.org/@alice/property-type/blurb"]
}
```

The second, cyclic version of the `Country` entity type

```json
{
  "kind": "entityType",
  // Because of versioning, we cannot change this version to /v/1 and create a "proper" cycle.
  "$id": "https://blockprotocol.org/@alice/entity-type/country/v/2",
  "type": "object",
  "title": "Region",
  "allOf": [
    { "$ref": "https://blockprotocol.org/@alice/entity-type/region/v/1" }
  ],
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/location": {
      "$ref": "https://blockprotocol.org/@alice/property-type/location/v/1"
    }
  },
  "required": ["https://blockprotocol.org/@alice/property-type/name"]
}
```

This sort of type hierarchy should _not_ be accepted within the type extension system, as the circular dependencies make types difficult to resolve/reason about. While the type hierarchy might be completely valid (as it would be in this case), we should safeguard users from making redundant type structures that look like the above.

In this specific contrived example, creating a new entity type based on `Region` instead of a new version of `Country` might even encode semantic meaning better than re-defining `Country`.

## Multiple supertypes - checking compatibility

As described in the [Guide-level explanation](#multiple-supertypes), when extending multiple entity types, they must be able to coexist in a "compatible manner", which means that the entity types' `properties` and `links` comply with the following:

- For each property (base URI on the top level of `properties`) that exists in multiple entity types:

  - all entity types refer to the same versioned URI of the property **or** compatible versions of the property (through the [versioning RFC](./0408-versioning-types.md#determining-type-compatibility) definition of 'compatible')
  - none of the entity types define the property as an array **or** all define the property as an array with the _exact same_ cardinality constraints.

- For each link (versioned URI on the top level of `links`) that exists in multiple entity types:

  - none of the entity types define the link as an array **or** all define the property as an array with the _exact same_ cardinality constraints (including `order`).

## Block Protocol implications

As mentioned [here](#addressing-previous-considerations), we want to make use of complete schemas for updates instead of partial schemas. In practice for the Block Protocol this makes it so the following update request using a partial schema:

```json
{
  // Old, partial updateEntityType message
  "entityTypeId": "https://blockprotocol.org/@alice/entity-type/person",
  "schema": {
    // The properties here are partially applied to the original Entity Type.
    "properties": {
      "https://blockprotocol.org/@alice/property-type/birth-date": {
        "$ref": "https://blockprotocol.org/@alice/property-type/birth-date/v/1"
      }
    }
  }
}
```

must instead use a complete schema

```json
{
  // New, complete updateEntityType message
  "entityTypeId": "https://blockprotocol.org/@alice/entity-type/person",
  "schema": {
    "$id": "https://blockprotocol.org/@alice/entity-type/person/v/2",
    "type": "object",
    "kind": "entityType",
    "title": "Person",
    "properties": {
      "https://blockprotocol.org/@alice/property-type/name/": {
        "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
      },
      "https://blockprotocol.org/@alice/property-type/email": {
        "$ref": "https://blockprotocol.org/@alice/property-type/email/v/1"
      },
      "https://blockprotocol.org/@alice/property-type/phone-number": {
        "$ref": "https://blockprotocol.org/@alice/property-type/phone-number/v/1"
      },
      // The update is inlined in the existing schema
      "https://blockprotocol.org/@alice/property-type/birth-date": {
        "$ref": "https://blockprotocol.org/@alice/property-type/birth-date/v/1"
      }
    }
  }
}
```

given that the original schema was created as follows

```json
{
  "$id": "https://blockprotocol.org/@alice/entity-type/person/v/1",
  "type": "object",
  "kind": "entityType",
  "title": "Person",
  "properties": {
    "https://blockprotocol.org/@alice/property-type/name": {
      "$ref": "https://blockprotocol.org/@alice/property-type/name/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/email": {
      "$ref": "https://blockprotocol.org/@alice/property-type/email/v/1"
    },
    "https://blockprotocol.org/@alice/property-type/phone-number": {
      "$ref": "https://blockprotocol.org/@alice/property-type/phone-number/v/1"
    }
  }
}
```

The above change applies to these BP operations (`updateEntityType` having already been implemented, and the others proposed in accepted RFCs):

- `updateEntityType`
- `updatePropertyType`
- `updateLinkType`

which must all make use of complete schemas in place of partial ones. These examples originate from the [Type System RFC](./0352-graph-type-system.md#interfacing-with-types-1).

While not explained in detail in the Type System RFC the `aggregateEntities` operation would need to change behavior slightly. The types returned from aggregating on a specific entity type could now also be implicitly coerced instances of entities (supertypes projected from subtype instances).

# Drawbacks

[drawbacks]: #drawbacks

- The way this proposal adds type extension means that we must implement some version of property selection/projection for types, which comes with non-trivial implementation details for embedding applications.
- This drifts further away from JSON Schema by introducing a different meaning to the `unevaluatedProperties` keyword and using it implicitly for our schemas.
- The way compatibility is defined could result in newer versions of supertypes becoming incompatible over time. This is not unique to extending types and can be an "issue" in many parts of the type system as different versions of property types become incompatible.

# Rationale and alternatives

[rationale-and-alternatives]: #rationale-and-alternatives

The general rationale for this way of handling extended types (which we may also call type inheritance) is that we want to keep supertypes and subtypes compatible with one another. Constraining the way type inheritance works makes it so that we can implicitly have "same as" relations across type inheritance trees. It also allows extending multiple supertypes which can lead to more expressive domain models.

## Problems and alternatives

`unevaluatedProperties` _almost_ provides the functionality we're after, but unfortunately it just barely misses. If supertypes themselves specify `{ "unevaluatedProperties": false }`, they are not able to be part of an `allOf` validator, as they will error out as soon as they see properties that are not part of the supertype itself.

When composing schemas that all contain `{ "unevaluatedProperties": false }`, each schema will disallow any other properties which they do not define. Using the following JSON Schema as an example:

```json
{
  "allOf": [
    {
      "type": "object",
      "properties": {
        "city": { "type": "string" }
      },
      "required": ["city"],
      "unevaluatedProperties": false
    }
  ],
  "type": "object",
  "properties": {
    "type": { "name": "string" }
  },
  "unevaluatedProperties": false
}
```

and trying to validate against

```json
{
  "city": "Copenhagen",
  "name": "Charles"
}
```

results in validations errors such as `Property 'name' has not been successfully evaluated and the schema does not allow unevaluated properties`.

Changing from `unevaluatedProperties` to `additionalProperties` results in errors `Property 'name' has not been defined and the schema does not allow additional properties`.

Both of these solutions for strict schemas would not be suitable for the type of expressiveness we want for type extension, unfortunately.

The required behavior is that `unevaluatedProperties` should only validate at the top level of a type, allowing supertypes to validate `unevaluatedProperties` but defer checking if they're used within in subtype.

### Alternative through open/closed schemas

A proposed way to deal with conditional `unevaluatedProperties` is to use custom `$defs` definitions for open and closed variations of entity types.

**Open by default schema**:

```json
{
  "$id": "https://example.com/schema",

  // ... schema contents  ...

  "$defs": {
    "closed": {
      "$anchor": "closed",
      "$ref": "#",
      "unevaluatedProperties": false
    }
  }
}
```

Here, referencing `https://example.com/schema` in a `$ref` will result in an _open schema_ that _does not_ specify `{ "unevaluatedProperties": false }`. Referencing `https://example.com/schema#closed` results in retrieving a _closed schema_ that _does_ specify `{ "unevaluatedProperties": false }`.

**Closed by default schema**:

```json
{
  "$id": "https://example.com/schema",
  "$ref": "#open",
  "unevaluatedProperties": false,

  "$defs": {
    "schema": {
      "$anchor": "open"
      // ... schema contents ...
    }
  }
}
```

Here, referencing `https://example.com/schema` in a `$ref` will result in a _closed schema_ that _does_ specify `{ "unevaluatedProperties": false }`. Referencing `https://example.com/schema#open` results in retrieving an _open schema_ that _does not_ specify `{ "unevaluatedProperties": false }`.

(Thanks Jason Desrosiers for the suggestions!)

Using the above setup would mean that we need to specify `#open` at the end of type URIs when URIs appear in `allOf`. We would also need to serve type schemas with one of the above structures, such that they conform with JSON Schema. Although this does add some ceremony around extending types, we would end up with a type extension system that would conform to JSON Schema without implicitness or redefining keywords.

# Prior art

[prior-art]: #prior-art

- [JSON Schema composition](https://json-schema.org/understanding-json-schema/reference/combining.html) and [`unevaluatedProperties`](https://json-schema.org/understanding-json-schema/reference/object.html#unevaluated-properties)
- [Programming languages subtyping](https://en.wikipedia.org/wiki/Subtyping)
- [Liskov substitution principle](https://en.wikipedia.org/wiki/Liskov_substitution_principle)

# Unresolved questions

[unresolved-questions]: #unresolved-questions

- We haven't specified how projecting/selecting properties of a supertype from a subtype instance is possible. It is an open question how we actually pick out the exact properties of a subtype to provide a valid supertype instance in embedding applications.
- Duplicating types of "forking" is not solved by this RFC.
- The current argument for not allowing cyclic type hierarchies mostly build on a feeling that type hierarchies shouldn't be too indirect/obfuscated, but there could be stronger arguments for allowing/disallowing it.

# Future possibilities

[future-possibilities]: #future-possibilities

- The conservative way type extension is introduced in this PR allows for future work in the "data mapping" space to apply to type inheritance as well. Being able to map between entity types could enable a shared mapping for otherwise incompatible (as supertypes) entity types.
- Implementations of the upcoming "Structure-based Queries" RFC could benefit from some of the groud-work set out by this RFC, as the selection/projection of supertypes could be the basis of structure-based queries.