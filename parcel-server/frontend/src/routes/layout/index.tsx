import * as React from "react";
import { Outlet, useMatches } from "react-router-dom";
import styled from "styled-components";
import Header from "./header";
import Footer from "./footer";
import PageTitle from "../../components/page_title";
import ContentBox from "./content_box";
import { RouteHandle } from "..";

const Wrapper = styled.div`
  height: 100%;
  display: grid;
  grid-template-rows: auto 1fr;
  grid-template-areas: "header" "content";
`;

const OuterContent = styled.div`
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
  const matches = useMatches();
  const match = matches[matches.length - 1];
  const handle = match.handle as RouteHandle;

  return (
    <Wrapper>
      <Header />
      <OuterContent>
        {!handle?.noContentWrapper ? (
          <Content>
            <CenterContainer>
              <PageTitle />
              <ContentBox>
                <Outlet />
              </ContentBox>
            </CenterContainer>
          </Content>
        ) : (
          <Outlet />
        )}
        {!handle?.noFooter && <Footer />}
      </OuterContent>
    </Wrapper>
  );
};

export default Layout;
