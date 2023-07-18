import * as React from "react";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

const Wrapper = styled.span`
  padding: 0.25rem 0.4rem;
  background: ${Colors.grayDark.gray8};
  margin: 0 0.2rem;
  font-size: 0.8rem;

  &:first-child {
    margin-left: 0;
  }

  &:last-child {
    margin-right: 0;
  }
`;

const Tag = ({ children }: React.PropsWithChildren) => {
  return <Wrapper>{children}</Wrapper>;
};

export default Tag;
