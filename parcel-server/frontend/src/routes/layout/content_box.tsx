import * as React from "react";
import styled from "styled-components";

const Wrapper = styled.div`
  padding: 1.5rem;
  border-radius: 1.5rem;
  background: #1a1c39;
  box-shadow: 0px 14px 36px 0px rgba(35, 51, 102, 0.3);
  border: 1px solid black;
`;

const ContentBox: React.FC<React.PropsWithChildren> = ({
  children,
}: React.PropsWithChildren) => {
  return <Wrapper>{children}</Wrapper>;
};

export default ContentBox;
