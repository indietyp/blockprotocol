import { ExpandedBlockMetadata } from "../../blocks";
import { connectToDatabase } from "../mongodb";

const collectionName = "bp-blocks";

const queryOptions = { projection: { _id: 0 } };

export const getDbBlocks = async (filter: { shortname?: string }) => {
  const { db } = await connectToDatabase();

  return db
    .collection<ExpandedBlockMetadata>(collectionName)
    .find(filter, queryOptions)
    .toArray();
};

export const getDbBlock = async ({
  name,
  shortname,
}: {
  name: string;
  shortname: string;
}) => {
  const { db } = await connectToDatabase();

  return db
    .collection<ExpandedBlockMetadata>(collectionName)
    .findOne({ author: shortname, name }, queryOptions);
};

export const insertDbBlock = async (block: ExpandedBlockMetadata) => {
  const { db } = await connectToDatabase();

  await db.collection<ExpandedBlockMetadata>(collectionName).insertOne(block);

  return block;
};

export const updateDbBlock = async (block: ExpandedBlockMetadata) => {
  const { db } = await connectToDatabase();

  const { author, name } = block;

  const { value: updatedBlock } = await db
    .collection<ExpandedBlockMetadata>(collectionName)
    .findOneAndUpdate(
      { author, name },
      { $set: block },
      { returnDocument: "after", ...queryOptions },
    );

  return updatedBlock;
};
