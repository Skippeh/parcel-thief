import * as Dialog from "@radix-ui/react-dialog";
import * as Colors from "@radix-ui/colors";
import styled, { keyframes } from "styled-components";

// no styling necessary for these
export const Root = Dialog.Root;
export const Trigger = Dialog.Trigger;
export const Portal = Dialog.Portal;

const overlayShow = keyframes`
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
`;

const contentShow = keyframes`
0% {
  opacity: 0;
  transform: translate(-50%, -48%) scale(0.95);
}

100% {
  opacity: 1;
  transform: translate(-50%, -50%) scale(1);
}
`;

export const Overlay = styled(Dialog.Overlay)`
  background: ${Colors.blackA.blackA9};
  animation: ${overlayShow} 0.22s cubic-bezier(0, 1, 1, 1);
  display: block;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
`;
export const Content = styled(Dialog.Content)`
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: ${Colors.indigoDark.indigo5};
  animation: ${contentShow} 0.25s cubic-bezier(0, 1, 1, 1);
  padding: 1.5rem;
  border-radius: 4px;
`;

export const Close = styled(Dialog.Close)`
  display: block;
  margin: 0;
  margin-top: 1rem;
`;

export const Title = styled(Dialog.Title)`
  margin-top: 0;
`;
