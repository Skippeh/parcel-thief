import * as React from "react";
import { Outlet } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";
import Footer from "./footer";
import PageTitle from "../../components/page_title";
import ContentBox from "./content_box";

const Wrapper = styled.div`
  height: 100%;
`;

const Content = styled.div`
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
          <PageTitle />
          <ContentBox>
            <Outlet />
          </ContentBox>
        </CenterContainer>
      </Content>
      <Footer />
    </Wrapper>
  );
};

export default Layout;
