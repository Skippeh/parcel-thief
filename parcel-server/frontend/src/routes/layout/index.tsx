import * as React from "react";
import { Outlet } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";
import Footer from "./footer";

const Wrapper = styled.div`
  height: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 0fr 0fr 0fr;
  grid-column-gap: 0;
  grid-row-gap: 0;
`;

const Content = styled.div`
  grid-area: 2 / 1 / 2 / 2;
  margin: 2rem;
  display: flex;
  justify-content: center;
`;

export const CenterContainer = styled.div`
  width: 100%;
  height: 100%;
  max-width: 1440px;
`;

const Layout = () => {
  return (
    <Wrapper>
      <Header />
      <Content>
        <CenterContainer>
          <Outlet />
        </CenterContainer>
      </Content>
      <Footer />
    </Wrapper>
  );
};

export default Layout;
