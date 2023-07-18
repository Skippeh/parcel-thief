import * as React from "react";
import { PropsWithChildren } from "react";
import styled from "styled-components";
import Breadcrumb from "./breadcrumb";

const Wrapper = styled.div`
  margin: 0.67rem 0;

  padding: 0.5rem 1rem;
  & h1 {
    padding: 0;
    margin: 0;
  }
`;

const PageTitle = ({ children }: PropsWithChildren) => {
  return (
    <Wrapper>
      <h1>{children}</h1>
      <Breadcrumb />
    </Wrapper>
  );
};

export default PageTitle;
