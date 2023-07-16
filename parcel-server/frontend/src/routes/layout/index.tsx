import * as React from "react";
import { Outlet } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";
import Footer from "./footer";

const Wrapper = styled.div`
  height: 100%;
`;

const Content = styled.div`
  margin: 1rem 0;
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
