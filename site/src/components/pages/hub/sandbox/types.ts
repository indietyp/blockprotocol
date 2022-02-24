import { JSONObject } from "blockprotocol";

import { ExpandedBlockMetadata } from "../../../../lib/blocks";

export type BlockInitMsg = {
  type: "initBlock";
  payload: string;
};

export type BlockInitData = {
  metadata: ExpandedBlockMetadata;
  schema: JSONObject;
  stringifiedSource: string;
};
