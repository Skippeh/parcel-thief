import * as React from "react";
import { Outlet } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";

const Wrapper = styled.div`
  height: 100%;
  display: grid;
  grid-template-columns: 20rem 1fr;
  grid-template-rows: 0fr 1fr;
  grid-column-gap: 0;
  grid-row-gap: 0;
`;

const Sidebar = styled.div`
  grid-area: 2 / 1 / 2 / 1;

  background: rgb(27, 33, 49);
  border-right: 1px solid rgb(41, 45, 57);
`;

const Content = styled.div`
  grid-area: 2 / 2 / 2 / 2;
`;

const Layout = () => {
  return (
    <Wrapper>
      <Header />
      <Sidebar>Sidebar</Sidebar>
      <Content>
        <Outlet />
      </Content>
    </Wrapper>
  );
};

export default Layout;
