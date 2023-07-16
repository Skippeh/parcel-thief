import * as React from "react";
import { styled } from "styled-components";
import { CenterContainer } from ".";
import GithubIcon from "./github-mark-white.svg";
import * as Colors from "@radix-ui/colors";

const Wrapper = styled.div`
  grid-area: 3 / 1 / 3 / 2;
  display: flex;
  justify-content: center;
`;

const InnerContainer = styled.div`
  display: flex;
  justify-content: center;
  font-size: 0.8rem;

  & span {
    vertical-align: super;
    color: ${Colors.whiteA.whiteA12};
  }

  & img {
    height: 3ex;
    padding-right: 0.25rem;
  }
`;

const Footer = () => {
  return (
    <Wrapper>
      <CenterContainer>
        <InnerContainer>
          <a href="https://github.com/Skippeh/parcel-thief" target="_blank">
            <img src={GithubIcon} alt="GitHub" />
            <span>Parcel Server</span>
          </a>
        </InnerContainer>
      </CenterContainer>
    </Wrapper>
  );
};

export default Footer;
