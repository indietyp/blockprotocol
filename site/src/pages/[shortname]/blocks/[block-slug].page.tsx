import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
import {
  Box,
  Breadcrumbs,
  Container,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import { formatDistance } from "date-fns";
import { GetStaticPaths, GetStaticProps, NextPage } from "next";
import Head from "next/head";
import { useRouter } from "next/router";
import React, { VoidFunctionComponent } from "react";

import { BlocksSlider } from "../../../components/blocks-slider";
import { FontAwesomeIcon } from "../../../components/icons";
import { Link } from "../../../components/link";
import { BlockDataContainer } from "../../../components/pages/hub/block-data-container";
import { BlockSchema } from "../../../components/pages/hub/hub-utils";
import {
  excludeHiddenBlocks,
  ExpandedBlockMetadata as BlockMetadata,
  readBlockDataFromDisk,
  readBlocksFromDisk,
} from "../../../lib/blocks";
import { isProduction } from "../../../lib/config";

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
const generateSandboxBaseUrl = (): string => {
  if (isProduction) {
    const deploymentUrl =
      process.env.NEXT_PUBLIC_BLOCK_SANDBOX_URL ??
      process.env.NEXT_PUBLIC_VERCEL_URL;

    if (!deploymentUrl) {
      throw new Error(
        "Could not generate frame origin: production environment detected but no process.env.NEXT_PUBLIC_BLOCK_SANDBOX_URL or process.env.NEXT_PUBLIC_VERCEL_URL",
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
  return `https://${branchPrefix}.vercel.app`;
};

const Bullet: VoidFunctionComponent = () => {
  return (
    <Box component="span" mx={1.5} sx={{ color: "#DDE7F0" }}>
      •
    </Box>
  );
};

type BlockPageProps = {
  blockMetadata: BlockMetadata;
  sandboxBaseUrl: string;
  schema: BlockSchema;
  sliderItems: BlockMetadata[];
};

type BlockPageQueryParams = {
  shortname?: string[];
  "block-slug"?: string;
};

export const getStaticPaths: GetStaticPaths<BlockPageQueryParams> = () => {
  return {
    paths: readBlocksFromDisk().map((metadata) => metadata.blockPackagePath),
    fallback: "blocking",
  };
};

const parseQueryParams = (params: BlockPageQueryParams) => {
  const shortname = params.shortname
    ? typeof params.shortname === "string"
      ? params.shortname
      : params.shortname.length === 1
      ? params.shortname[0]
      : undefined
    : undefined;

  if (!shortname) {
    throw new Error("Could not parse org shortname from query");
  }

  const blockSlug = params["block-slug"];

  if (!blockSlug) {
    throw new Error("Could not parse block slug from query");
  }

  return { shortname, blockSlug };
};

/**
 * Helps display `github.com/org/repo` instead of a full URL with protocol, commit hash and path.
 * If a URL is not recognised as a GitHub repo, only `https://` is removed.
 */
const generateRepositoryDisplayUrl = (repository: string): string => {
  const repositoryUrlObject = new URL(repository);
  const displayUrl = `${repositoryUrlObject.hostname}${repositoryUrlObject.pathname}`;

  if (repositoryUrlObject.hostname === "github.com") {
    return displayUrl.split("/").slice(0, 3).join("/");
  }

  return displayUrl;
};

export const getStaticProps: GetStaticProps<
  BlockPageProps,
  BlockPageQueryParams
> = async ({ params }) => {
  const { shortname, blockSlug } = parseQueryParams(params || {});

  if (!shortname.startsWith("@")) {
    return { notFound: true };
  }

  const packagePath = `${shortname}/${blockSlug}`;
  const catalog = readBlocksFromDisk();

  const blockMetadata = catalog.find(
    (metadata) => metadata.packagePath === packagePath,
  );

  if (!blockMetadata) {
    // TODO: Render custom 404 page for blocks
    return { notFound: true };
  }

  const { schema } = await readBlockDataFromDisk(blockMetadata);

  return {
    props: {
      blockMetadata,
      sliderItems: excludeHiddenBlocks(catalog).filter(
        ({ name }) => name !== blockMetadata.name,
      ),
      sandboxBaseUrl: generateSandboxBaseUrl(),
      schema,
    },
    revalidate: 1800,
  };
};

const BlockPage: NextPage<BlockPageProps> = ({
  blockMetadata,
  sandboxBaseUrl,
  schema,
  sliderItems,
}) => {
  const { query } = useRouter();
  const { shortname } = parseQueryParams(query || {});

  const theme = useTheme();

  const md = useMediaQuery(theme.breakpoints.up("md"));
  const isDesktopSize = md;

  const repositoryDisplayUrl = blockMetadata.repository
    ? generateRepositoryDisplayUrl(blockMetadata.repository)
    : "";

  return (
    <>
      <Head>
        <title>
          Block Protocol - {blockMetadata.displayName} Block by {shortname}
        </title>
      </Head>
      <Container>
        {isDesktopSize ? null : (
          <Box mb={1}>
            <Breadcrumbs
              separator={
                <FontAwesomeIcon
                  icon={faChevronRight}
                  sx={{
                    fontSize: 14,
                    color: ({ palette }) => palette.gray[40],
                  }}
                />
              }
            >
              <Link href="/">Home</Link>
              <Link href="/hub">Block Hub</Link>
              <Typography variant="bpSmallCopy" color="inherit">
                {blockMetadata.displayName}
              </Typography>
            </Breadcrumbs>
          </Box>
        )}

        <Box
          sx={{ display: "flex", pt: { xs: 4, md: 10 }, mb: { xs: 6, md: 12 } }}
        >
          {isDesktopSize ? (
            <Typography variant="bpHeading1">
              <Box
                sx={{
                  display: "inline-block",
                  mr: 3,
                  height: "2em",
                  width: "2em",
                }}
                component="img"
                src={blockMetadata.icon ?? undefined}
              />
            </Typography>
          ) : null}

          <Box>
            <Typography
              sx={{ display: { xs: "flex", md: "unset" } }}
              variant="bpHeading1"
              mt={2}
            >
              {!isDesktopSize && (
                <Box
                  sx={{
                    display: "inline-block",
                    height: "1em",
                    width: "1em",
                    mr: 2,
                  }}
                  component="img"
                  src={blockMetadata.icon ?? undefined}
                />
              )}
              {blockMetadata.displayName}
            </Typography>
            <Typography variant="bpBodyCopy">
              <Box sx={{ color: theme.palette.gray[80] }}>
                {blockMetadata.description}
              </Box>
            </Typography>
            <Typography
              variant="bpSmallCopy"
              sx={{
                color: ({ palette }) => palette.gray[70],
              }}
            >
              <span>
                By{" "}
                <Box
                  component="span"
                  sx={{
                    color: ({ palette }) => palette.purple[700],
                  }}
                >
                  <Link href={`/${shortname}`}>{shortname}</Link>
                </Box>
              </span>
              <Bullet />
              <span>V{blockMetadata.version}</span>
              {blockMetadata.lastUpdated ? (
                <>
                  {isDesktopSize && <Bullet />}
                  <Box
                    component="span"
                    sx={{ display: { xs: "block", md: "inline-block" } }}
                  >
                    {`Updated ${formatDistance(
                      new Date(blockMetadata.lastUpdated),
                      new Date(),
                      {
                        addSuffix: true,
                      },
                    )}`}
                  </Box>
                </>
              ) : null}
            </Typography>
          </Box>
        </Box>

        <Box sx={{ mb: 10 }}>
          <BlockDataContainer
            metadata={blockMetadata}
            schema={schema}
            sandboxBaseUrl={sandboxBaseUrl}
          />
        </Box>

        {blockMetadata.repository && (
          <Box
            mb={10}
            sx={{
              display: "grid",
              gridTemplateColumns: { xs: "1fr", md: "60% 40%" },
            }}
          >
            <Box />
            <Box sx={{ overflow: "hidden" }} pl={{ xs: 0, md: 2 }}>
              <Typography
                variant="bpLargeText"
                sx={{
                  fontWeight: "bold",
                  color: theme.palette.gray[80],
                  marginBottom: 2,
                }}
              >
                Repository
              </Typography>
              <Box sx={{ display: "flex" }}>
                <Box
                  component="img"
                  alt="GitHub Link"
                  sx={{ marginRight: 1.5 }}
                  src="/assets/link.svg"
                />{" "}
                <Typography
                  variant="bpSmallCopy"
                  sx={{ overflow: "hidden", textOverflow: "ellipsis" }}
                >
                  <Link href={blockMetadata.repository}>
                    {repositoryDisplayUrl}
                  </Link>
                </Typography>
              </Box>
            </Box>
          </Box>
        )}

        {/* <div
        style={{ display: "grid", gridTemplateColumns: "60% 40%" }}
        className=" mb-10"
      >
        <div>
          <b>About</b>
          <p>
            Store information in rows and columns in a classic table layout.
            Longer description talking about parameters and how to use like a
            readme goes in here. Tables have filters, search, ability to add and
            remove columns and rows, multiple views. Tables have filters,
            search, ability to add and remove columns and rows, multiple views.
            Tables have filters, search, ability to add and remove columns and
            rows, multiple views. Tables have filters, search, ability to add
            and remove columns and rows, multiple views.
          </p>
        </div>
        
      </div> */}

        <Typography textAlign="center" variant="bpHeading2" mb={3}>
          Explore more blocks
        </Typography>
      </Container>
      <BlocksSlider catalog={sliderItems} />
    </>
  );
};

export default BlockPage;