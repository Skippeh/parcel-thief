import * as React from "react";
import styled from "styled-components";
import Breadcrumb from "./breadcrumb";
import { useMatches } from "react-router-dom";
import { RouteHandle } from "../routes";

const Wrapper = styled.div`
  margin: 0.67rem 0;

  padding: 0.5rem 1rem;
  & h1 {
    padding: 0;
    margin: 0;
  }
`;

const PageTitle = () => {
  const matches = useMatches();
  let match;

  // Iterate matches backwards until we find one with a handle defined
  for (let i = matches.length - 1; i >= 0; i--) {
    match = matches[i];

    if (match.handle != null) {
      break;
    }
  }

  const handle = match.handle as RouteHandle;
  let title: string | undefined = undefined;

  if (handle != null) {
    title = handle.title ?? handle.crumb;
  }

  return (
    <Wrapper>
      <h1>{title}</h1>
      <Breadcrumb />
    </Wrapper>
  );
};

export default PageTitle;
