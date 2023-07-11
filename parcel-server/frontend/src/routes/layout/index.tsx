import * as React from "react";
import { Outlet } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";

const Wrapper = styled.div`
  height: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 0fr 1fr;
  grid-column-gap: 0;
  grid-row-gap: 0;
`;

const Content = styled.div`
  grid-area: 2 / 1 / 2 / 2;
`;

const Layout = () => {
  return (
    <Wrapper>
      <Header />
      <Content>
        <Outlet />
      </Content>
    </Wrapper>
  );
};

export default Layout;
