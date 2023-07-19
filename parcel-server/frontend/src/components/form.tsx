import * as React from "react";
import * as Form from "@radix-ui/react-form";
import * as Colors from "@radix-ui/colors";
import { styled } from "styled-components";

export const Root = styled(Form.Root)``;
export const Field = styled(Form.Field)`
  display: flex;
  flex-direction: column;
  gap: 0.4rem;

  &:not(:last-of-type) {
    margin-bottom: 0.4rem;
  }
`;
export const Control = styled(Form.Control)`
  width: 100%;
`;
export const Label = styled(Form.Label)`
  font-weight: bold;
  font-size: 0.9rem;
`;
export const Message = styled(Form.Message)``;
export const ValidityState = styled(Form.ValidityState)``;
export const Submit = styled(Form.Submit)`
  margin-top: 1.5rem;
`;
