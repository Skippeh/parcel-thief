import * as React from "react";
import { styled } from "styled-components";

const Wrapper = styled.div`
  width: 600px;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
`;

const BrightBlue = "#1687d8c5";
const Blue = "#0e85c092";
const LabelHeader = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;

  & .arrow {
    display: inline-block;
    width: 0;
    height: 0;
    border-top: 15px solid transparent;
    border-right: 10px solid ${BrightBlue};
    border-bottom: 15px solid transparent;
  }

  & .inner {
    width: 100%;
    display: inline-flex;
    align-items: center;
    background: linear-gradient(
      to right,
      ${BrightBlue} 0%,
      ${BrightBlue} 60%,
      ${Blue} 100%
    );
    border: 1px solid ${BrightBlue};
    border-left: none;
    height: 30px;

    & .label {
      padding-left: 0.25rem;
      display: inline;
    }
  }
`;

const Content = styled.div`
  background: #000000c5;
  border: 1px solid ${Blue};
  margin-left: 10px;
  padding: 0.5rem;
`;

interface RootProps extends React.PropsWithChildren {
  label: string;
}

export default ({ label, children }: RootProps) => {
  return (
    <Wrapper>
      <LabelHeader>
        <div className="arrow" />
        <div className="inner">
          <span className="label">{label}</span>
        </div>
      </LabelHeader>
      <Content>{children}</Content>
    </Wrapper>
  );
};
