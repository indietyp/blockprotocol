import { faBookOpen } from "@fortawesome/free-solid-svg-icons";
import { ReactElement } from "react";

import { SiteMapPage, SiteMapPageSection } from "../../lib/sitemap";
import { BlockHubIcon, FontAwesomeIcon, SpecificationIcon } from "../icons";

export const itemIsPage = (
  item: SiteMapPage | SiteMapPageSection,
): item is SiteMapPage => "href" in item;

export const NAVBAR_LINK_ICONS: Record<string, ReactElement> = {
  "Block Hub": (
    <BlockHubIcon
      sx={{
        width: 18,
        height: 18,
      }}
    />
  ),
  Documentation: (
    <FontAwesomeIcon
      icon={faBookOpen}
      sx={{
        fontSize: 18,
      }}
    />
  ),
  Specification: (
    <SpecificationIcon
      sx={{
        width: 18,
        height: 18,
      }}
    />
  ),
};
