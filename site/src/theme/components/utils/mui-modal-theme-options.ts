import { Components, Theme } from "@mui/material";

export const MuiModalThemeOptions: Components<Theme>["MuiModal"] = {
  defaultProps: {
    componentsProps: {
      backdrop: {
        style: {
          background:
            "radial-gradient(141.84% 147.92% at 50% 122.49%, rgba(210, 114, 255, 0.7) 0%, rgba(148, 130, 255, 0.7) 55.21%, rgba(35, 57, 255, 0.7) 100%)",
          backdropFilter: "blur(4px)",
        },
      },
    },
  },
};
