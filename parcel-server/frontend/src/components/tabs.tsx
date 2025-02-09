import * as React from "react";
import * as Tabs from "@radix-ui/react-tabs";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

export const Root = styled(Tabs.Root)``;
export const List = styled(Tabs.List)``;
export const Trigger = styled(Tabs.Trigger)`
  all: unset;
  box-sizing: border-box;
  padding: 0.8rem 1rem;
  cursor: pointer;
  background: ${Colors.blueDark.blue6};
  transition: background-color 0.1s ease-out;
  font-size: 0.82rem;

  &:first-child {
    border-top-left-radius: 4px;
  }

  &:last-child {
    border-top-right-radius: 4px;
  }

  &[data-state="active"] {
    background: ${Colors.blueDark.blue8};
  }

  &:not([data-state="active"]):hover {
    background: ${Colors.blueDark.blue7};
  }
`;
export const Content = styled(Tabs.Content)<{ $padded?: boolean }>`
  & > *:first-child {
    margin-top: ${(props) => (props.$padded ? "1.5rem" : "inherit")};
  }

  &[data-state="inactive"] {
    display: none;
  }
`;
