import * as React from "react";
import { FrontendPermissions } from "../../../../../api_types";
import {
  permissionToReadableString,
  setAccountPermissions,
} from "../../../../../services/accounts_service";
import styled from "styled-components";
import * as Form from "../../../../../components/form";
import { useState } from "react";
import SaveButton from "../../../../../components/save_button";
import { ApiResponse } from "../../../../../services";

const allPermissions: FrontendPermissions[] = [
  "manageAccounts",
  "manageServerSettings",
];
const permissionNames = Object.fromEntries(
  allPermissions.map((permission) => [
    permission,
    permissionToReadableString(permission),
  ])
);

const Fields = styled.div`
  display: flex;
  flex-wrap: wrap;
  max-width: 300px;
  //justify-content: space-between;
  gap: 0.5rem;
  row-gap: 0.1rem;
`;

const FormField = styled(Form.Field)`
  display: block;

  & input {
    width: unset;
    margin-right: 0.2rem;
  }

  & label {
    font-weight: normal;
  }

  // undo bottom margin set by parent style
  &:not(:last-of-type) {
    margin-bottom: unset;
  }
`;

const ErrorText = styled.span`
  margin-left: 0.2rem;
`;

interface Props {
  permissions: FrontendPermissions[];
  accountId: number;
  updatePermissions: (permissions: FrontendPermissions[]) => void;
}

const PermissionsEditor = ({
  permissions,
  accountId,
  updatePermissions,
}: Props) => {
  const [newPermissions, setNewPermissions] =
    useState<FrontendPermissions[]>(permissions);
  const [error, setError] = useState<string | null>(null);

  function onPermissionChanged(event: React.ChangeEvent<HTMLInputElement>) {
    const checked = event.target.checked;

    if (checked) {
      setNewPermissions([
        ...newPermissions,
        event.target.name as FrontendPermissions, // typecasting is safe because name always matches an enum variant
      ]);
    } else {
      setNewPermissions(
        newPermissions.filter((permission) => permission !== event.target.name)
      );
    }
  }

  async function onSave() {
    const response = await setAccountPermissions(accountId, newPermissions);

    if (response.error != null) {
      setError(response.error);
    } else if (response.data != null) {
      setError(null);
      updatePermissions(response.data);
    }

    return response;
  }

  React.useEffect(() => {
    setNewPermissions(permissions);
  }, [permissions]);

  return (
    <Form.Root>
      <Fields>
        {allPermissions.map((permission) => (
          <FormField name={permission} key={permission}>
            <Form.Control
              type="checkbox"
              name={permission}
              checked={newPermissions.includes(permission)}
              onChange={onPermissionChanged}
            />
            <Form.Label>{permissionNames[permission]}</Form.Label>
          </FormField>
        ))}
      </Fields>
      <SaveButton saveAction={onSave} isForm>
        Save
      </SaveButton>
      {error != null && <ErrorText className="error">{error}</ErrorText>}
    </Form.Root>
  );
};

export default PermissionsEditor;
