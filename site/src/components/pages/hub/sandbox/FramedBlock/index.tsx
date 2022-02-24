import React from "react";
import ReactDOM from "react-dom";
import { FramedBlock } from "./FramedBlock";

// For some reason this is not actually executing
// The file (blockSandbox.js) is created
// The file is loaded in the created sandbox.html
// But nothing happens - even if you replace all this with a console.log
// Something to do with how webpack5 now outputs the chunk?
ReactDOM.render(
  <React.StrictMode>
    <FramedBlock />
  </React.StrictMode>,
  document.getElementById("sandbox-root"),
);
