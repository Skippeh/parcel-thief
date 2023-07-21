import * as React from "react";
import { createRoot } from "react-dom/client";
import App from "./app";
import styled, { StyleSheetManager, WebTarget } from "styled-components";
import "normalize.css";
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-alpine.css";
import GlobalStyle from "./global_style";
import isPropValid from "@emotion/is-prop-valid";

const shouldForwardProp = (propName: string, target: WebTarget): boolean => {
  if (typeof target == "string" && !isPropValid(propName)) {
    console.warn(`Not forwarding prop '${propName}' on`, target);
    return false;
  }

  return true;
};

const rootElement = document.getElementById("root");

if (rootElement == null) {
  throw new Error("Failed to find root element");
}

const root = createRoot(rootElement);
root.render(
  <React.StrictMode>
    <StyleSheetManager shouldForwardProp={shouldForwardProp}>
      <GlobalStyle />
      <App />
    </StyleSheetManager>
  </React.StrictMode>
);
