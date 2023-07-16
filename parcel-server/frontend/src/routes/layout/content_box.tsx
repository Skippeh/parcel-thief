import * as React from "react";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

const Wrapper = styled.div`
  padding: 1.5rem;
  border-radius: 4px;
  background: ${Colors.indigoDark.indigo3};
  border: 1px solid black;
`;

const ContentBox: React.FC<React.PropsWithChildren> = ({
  children,
}: React.PropsWithChildren) => {
  return <Wrapper>{children}</Wrapper>;
};

export default ContentBox;
