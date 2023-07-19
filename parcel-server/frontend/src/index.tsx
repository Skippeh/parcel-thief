import * as React from "react";
import { createRoot } from "react-dom/client";
import App from "./app";
import styled, { StyleSheetManager } from "styled-components";
import "normalize.css";
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-alpine.css";
import GlobalStyle from "./global_style";
import isPropValid from "@emotion/is-prop-valid";

const rootElement = document.getElementById("root");

if (rootElement == null) {
  throw new Error("Failed to find root element");
}

const root = createRoot(rootElement);
root.render(
  <React.StrictMode>
    <StyleSheetManager shouldForwardProp={isPropValid}>
      <GlobalStyle />
      <App />
    </StyleSheetManager>
  </React.StrictMode>
);
