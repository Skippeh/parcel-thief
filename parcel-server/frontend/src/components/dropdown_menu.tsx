import * as React from "react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

export const Root = styled(DropdownMenu.Root)``;
export const Trigger = styled(DropdownMenu.Trigger)`
  all: unset;
`;
export const Portal = DropdownMenu.Portal; // no styling necessary
export const Content = styled(DropdownMenu.Content)`
  min-width: 200px;
  background: ${Colors.blueDark.blue2};
  border-radius: 4px;
  border: 1px solid ${Colors.whiteA.whiteA7};
`;
export const Item = styled(DropdownMenu.Item)`
  all: unset;
  cursor: pointer;
  display: block;
  padding: 0.5rem;
  font-size: 0.9rem;

  &:hover {
    background: ${Colors.blueDark.blue7};
  }
`;
