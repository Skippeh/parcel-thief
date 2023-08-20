import * as Popover from "@radix-ui/react-popover";
import styled from "styled-components";

export const Root = Popover.Root;
export const Trigger = styled(Popover.Trigger)`
  all: unset;
  box-sizing: border-box;

  &:focus,
  &:hover,
  &:active {
    all: unset;
  }
`;
export const Portal = Popover.Portal;
export const Content = Popover.Content;
export const Anchor = Popover.Anchor;
