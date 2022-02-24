import { useCallback, useEffect, useRef, VoidFunctionComponent } from "react";

import { BlockInitData, BlockInitMsg } from "../types";
import { isProduction } from "../../../../../lib/config";

type CrossFrameProxyProps = {
  blockInitData: BlockInitData;
};

/**
 * We want a different origin for the iFrame to the parent window
 * so that it can't use cookies issued to the user in the main app.
 *
 * The PRODUCTION origin will be blockprotocol.org, so we can use
 * the unique Vercel deployment URL as the origin in production.
 *
 * In STAGING, we will mostly be visiting unique deployment URLs
 * for testing, so we can use the unique branch URL as the origin.
 * Note: this means the frame in preview deployments will always be
 * built from the tip of the branch - if you visit the non-latest preview
 * deployment AND you have changed the framed code, they may be out of sync.
 */
const generateFrameOrigin = () => {
  if (isProduction) {
    const deploymentUrl = process.env.NEXT_PUBLIC_VERCEL_URL;

    if (!deploymentUrl) {
      throw new Error(
        "Could not generate frame origin: production environment detected but no process.env.NEXT_PUBLIC_VERCEL_URL",
      );
    }

    return deploymentUrl;
  }

  const branch = process.env.NEXT_PUBLIC_VERCEL_GIT_COMMIT_REF;

  if (!branch) {
    // eslint-disable-next-line no-console
    console.warn(
      "Running locally: block hub iFrame has same origin as main app. Block code can make authenticated requests to main app API.",
    );
    return "";
  }

  // @see https://vercel.com/docs/concepts/deployments/automatic-urls
  const slugifiedBranch = branch.toLowerCase().replace(/[^\w-]+/g, "-");
  const branchPrefix = `blockprotocol-git-${slugifiedBranch}-hashintel`.slice(
    0,
    64,
  );
  return `${branchPrefix}.vercel.app`;
};

export const BlockFramer: VoidFunctionComponent<CrossFrameProxyProps> = ({
  blockInitData,
}) => {
  const frameRef = useRef<HTMLIFrameElement>(null);

  const origin = generateFrameOrigin();

  const framePath = `${origin}/_next/static/sandbox.html`;

  const sendBlockData = useCallback(() => {
    const msg: BlockInitMsg = {
      payload: JSON.stringify(blockInitData),
      type: "initBlock",
    };
    frameRef.current?.contentWindow?.postMessage(msg, origin);
  }, [blockInitData, origin]);

  useEffect(() => {
    sendBlockData();
  }, [blockInitData, sendBlockData]);

  const onLoad = useCallback(() => {
    sendBlockData();
  }, [sendBlockData]);

  return (
    <iframe
      title="Block Sandbox"
      sandbox="allow-scripts allow-top-navigation-by-user-activation"
      frameBorder={0}
      onLoad={onLoad}
      src={framePath}
      style={{ minWidth: "100%", maxWidth: "100%", height: "500px" }}
    />
  );
};
