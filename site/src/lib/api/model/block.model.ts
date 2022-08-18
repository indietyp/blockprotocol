import { BlockMetadata, JsonObject } from "@blockprotocol/core";
import { Db, ObjectId, WithId } from "mongodb";

import localBlocks from "../../../../blocks-data.json" assert { type: "json" };
import { ExpandedBlockMetadata } from "../../blocks";
import { connectToDatabase } from "../mongodb";
import { User } from "./user.model";

const BUCKET_BASE_URL =
  "https://c6499786332a3d2fb35419a7803ab7aa.r2.cloudflarestorage.com/blocks";

type BlockSourceProperties = {
  registry: "npm";
  repository: string;
};

type BlockBuildStatus = "not_built" | "building" | "built";

type BlockDbProperties = {
  assetFolderPath: string;
  blockName: string;
  blockNamespace: string;
  buildStatus: BlockBuildStatus;
  createdAt: Date;
  lastBuiltAt: Date | null;
  metadata?: ExpandedBlockMetadata | null;
  source: BlockSourceProperties;
  updatedAt: Date;
};

export class Block {
  _id: ObjectId;
  assetFolderPath: string;
  blockName: string;
  blockNamespace: string;
  buildStatus: BlockBuildStatus;
  metadata?: ExpandedBlockMetadata | null;
  createdAt: Date;
  lastBuiltAt: Date | null;
  source: BlockSourceProperties;
  updatedAt: Date;

  static readonly COLLECTION_NAME = "bp-blocks";

  private constructor({
    _id,
    assetFolderPath,
    blockName,
    blockNamespace,
    buildStatus,
    metadata,
    lastBuiltAt,
    createdAt,
    updatedAt,
    source,
  }: WithId<BlockDbProperties>) {
    this._id = _id;
    this.blockName = blockName;
    this.blockNamespace = blockNamespace;
    this.source = source;

    this.assetFolderPath = assetFolderPath;
    this.buildStatus = buildStatus;
    this.metadata = metadata;
    this.lastBuiltAt = lastBuiltAt;

    this.createdAt = createdAt;
    this.updatedAt = updatedAt;
  }

  static async create(
    db: Db,
    params: {
      blockName: string;
      source: BlockSourceProperties;
      user: User;
    },
  ): Promise<Block> {
    const { blockName, source, user } = params;

    if (!user.shortname) {
      throw new Error(
        "User does not have a shortname. Cannot create block for user.",
      );
    }

    const blockNamespace = user.shortname;

    const assetFolderPath = `${BUCKET_BASE_URL}/${blockNamespace}/${blockName}`;

    const now = new Date();
    const block = {
      assetFolderPath,
      blockName,
      blockNamespace,
      cachedFileContent: null,
      buildStatus: "not_built" as "not_built",
      lastBuiltAt: null,
      createdAt: now,
      updatedAt: now,
      source,
    };

    const { insertedId: _id } = await db
      .collection<BlockDbProperties>(this.COLLECTION_NAME)
      .insertOne(block);

    return new Block({ _id, ...block });
  }

  static async getAll(db: Db): Promise<Block[]> {
    const blocksFromDb = await db
      .collection<BlockDbProperties>(this.COLLECTION_NAME)
      .find({})
      .toArray()
      .then((docs) => docs.map((doc) => new Block(doc)));

    return [...localBlocks, ...blocksFromDb];
  }

  static async getAllByUser(db: Db, params: { user: User }): Promise<Block[]> {
    return await db
      .collection<Block>(this.COLLECTION_NAME)
      .find(
        { blockNamespace: params.user.shortname },
        {
          projection: {
            _id: 0,
          },
        },
      )
      .toArray()
      .then((docs) => docs.map((doc) => new Block(doc)));
  }

  static async getByUserAndName(
    db: Db,
    params: { name: string; user: User },
  ): Promise<Block | null> {
    const block = await db
      .collection<BlockDbProperties>(this.COLLECTION_NAME)
      .findOne(
        { blockName: params.name, blockNamespace: params.user.shortname },
        {
          projection: {
            _id: 0,
          },
        },
      );

    return block ? new Block(block) : null;
  }

  async build() {
    const { db } = await connectToDatabase();

    await db.collection<Block>(Block.COLLECTION_NAME).findOneAndUpdate(
      {
        blockName: this.blockName,
        blockNamespace: this.blockNamespace,
      },
      { $set: { buildStatus: "building" } },
    );
  }

  async update(
    db: Db,
    params: { source?: BlockSourceProperties },
  ): Promise<Block> {
    const { source } = params;

    const now = new Date();

    const { value: updatedBlock } = await db
      .collection<Block>(Block.COLLECTION_NAME)
      .findOneAndUpdate(
        { blockName: this.blockName, blockNamespace: this.blockNamespace },
        {
          $set: {
            source: source ?? undefined,
            updatedAt: now,
          },
        },
        { returnDocument: "after" },
      );

    if (!updatedBlock) {
      throw new Error(
        "Critical: could not find record of Block instance in database.",
      );
    }

    return new Block(updatedBlock);
  }
}
