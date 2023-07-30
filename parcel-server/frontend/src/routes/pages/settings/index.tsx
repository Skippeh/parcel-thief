import * as React from "react";
import Code from "../../../components/code";
import * as Form from "../../../components/form";
import InfoText from "../../../components/info_text";
import SaveButton from "../../../components/save_button";
import { ApiResponse } from "../../../services";

const Settings = () => {
  const [error, setError] = React.useState<string | null>(null);

  const doSave = async (): Promise<ApiResponse<void>> => {
    setError("Not implemented");
    return {
      data: null,
      error: "Not implemented",
      formErrors: null,
      statusCode: 500,
    };
  };

  return (
    <>
      <Form.Root>
        <Form.Field name="publicServer">
          <Form.Label>Public server</Form.Label>
          <Form.SubLabel>
            <p>
              If checked, anyone who knows of the server address will be able to
              log in to the game server. Otherwise they must first be added to
              the whitelist.
            </p>
            <p>
              The whitelist can be edited by changing{" "}
              <Code>data/whitelist.txt</Code> in the server directory.
            </p>
          </Form.SubLabel>
          <Form.Control type="checkbox" />
        </Form.Field>
        <Form.Field name="allowFrontendLogin">
          <Form.Label>Allow any user to login to the frontend</Form.Label>
          <Form.SubLabel>
            If checked, any user with an existing game account can log in to the
            frontend. Otherwise an admin must first create a frontend account
            for the user.
          </Form.SubLabel>
          <Form.Control type="checkbox" />
        </Form.Field>
        <SaveButton isForm saveAction={doSave}>
          Save
        </SaveButton>
        {error != null && <span className="error">{error}</span>}
      </Form.Root>
    </>
  );
};

export default Settings;
