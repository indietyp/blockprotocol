import { useEffect, useMemo, useState, VoidFunctionComponent } from "react";

import { BlockDataContainer } from "../../BlockDataContainer";
import { BlockInitData, BlockInitMsg } from "../types";
import {
  blockDependencies,
  BlockDependency,
  BlockExports,
} from "../../HubUtils";

const blockRequire = (name: BlockDependency) => {
  if (!(name in blockDependencies)) {
    throw new Error(`missing dependency ${name}`);
  }

  return blockDependencies[name];
};

const blockEval = (source: string): BlockExports => {
  const exports_ = {};
  const module_ = { exports: exports_ };

  // eslint-disable-next-line no-new-func
  const moduleFactory = new Function("require", "module", "exports", source);
  moduleFactory(blockRequire, module_, exports_);

  return module_.exports as BlockExports;
};

export const FramedBlock: VoidFunctionComponent = () => {
  const [blockData, setBlockData] = useState<BlockInitData | null>(null);

  const blockModule = useMemo(
    () =>
      blockData?.stringifiedSource
        ? blockEval(blockData.stringifiedSource)
        : undefined,
    [blockData?.stringifiedSource],
  );

  useEffect(() => {
    const msgHandler = ({ data }: MessageEvent<BlockInitMsg>) => {
      switch (data.type) {
        case "initBlock":
          setBlockData(JSON.parse(data.payload));
          break;
      }
    };
    window.addEventListener("message", msgHandler);

    return () => window.removeEventListener("message", msgHandler);
  }, []);

  if (!blockData) {
    return <div>Loading...</div>;
  }

  return (
    <BlockDataContainer
      blockModule={blockModule}
      metadata={blockData.metadata}
      schema={blockData.schema}
    />
  );
};
