import * as React from "react";
import { Info } from "@phosphor-icons/react";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";
import * as Dialog from "./dialog";

const DialogTrigger = styled(Dialog.Trigger)`
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  gap: 0.2rem;
  font-size: 0.9rem;
  padding: 0.3rem;
  background: transparent;
  transition: background-color 0.1s ease-out;

  &:hover,
  &:focus-visible {
    background: ${Colors.grayDark.gray8};
    outline: none;
  }

  &:active {
    background: ${Colors.grayDark.gray7};
  }
`;

const InfoText = ({
  children,
  title,
}: React.PropsWithChildren<{ title?: string }>) => {
  return (
    <Dialog.Root>
      <DialogTrigger tabIndex={0}>
        <Info weight="fill" />
        <span>More info</span>
      </DialogTrigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>{title || "More info"}</Dialog.Title>
          {children}
          <Dialog.Close>Understood</Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default InfoText;
