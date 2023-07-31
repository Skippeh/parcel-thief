import * as React from "react";
import Code from "../../../components/code";
import * as Form from "../../../components/form";
import SaveButton from "../../../components/save_button";
import { ApiResponse } from "../../../services";
import { SettingsValues } from "../../../api_types";
import {
  getServerSettings,
  setServerSettings as setSettingsService,
} from "../../../services/settings_service";

const Settings = () => {
  const [error, setError] = React.useState<string | null>(null);
  const [settings, setSettings] = React.useState<
    SettingsValues | undefined | null
  >(undefined);

  async function doSave(): Promise<ApiResponse<SettingsValues>> {
    setError(null);
    const response = await setSettingsService(settings);

    if (response.data != null) {
      setSettings(response.data);
    } else {
      setError(response.error);
    }

    return response;
  }

  function setPublicServer(value: boolean) {
    setSettings({
      ...settings,
      publicServer: value,
    });
  }

  function setAllowFrontendLogin(value: boolean) {
    setSettings({
      ...settings,
      allowFrontendLogin: value,
    });
  }

  React.useEffect(() => {
    (async () => {
      if (settings === undefined) {
        const response = await getServerSettings();

        if (response.data != null) {
          setSettings(response.data);
        } else {
          setSettings(null);
          setError(response.error);
        }
      }
    })();
  }, []);

  return (
    <>
      {settings === undefined ? (
        "Loading..."
      ) : (
        <>
          <Form.Root>
            <Form.Field name="publicServer">
              <Form.Label>Public server</Form.Label>
              <Form.SubLabel>
                <p>
                  If checked, anyone who knows of the server address will be
                  able to log in to the game server. Otherwise they must first
                  be added to the whitelist.
                </p>
                <p>
                  The whitelist can be edited by changing{" "}
                  <Code>data/whitelist.txt</Code> in the server directory.
                </p>
              </Form.SubLabel>
              <Form.Control
                type="checkbox"
                checked={settings.publicServer}
                onChange={(ev) => setPublicServer(ev.target.checked)}
              />
            </Form.Field>
            <Form.Field name="allowFrontendLogin">
              <Form.Label>Allow any user to login to the frontend</Form.Label>
              <Form.SubLabel>
                If checked, any user with an existing game account can log in to
                the frontend. Otherwise an admin must first create a frontend
                account for the user.
              </Form.SubLabel>
              <Form.Control
                type="checkbox"
                checked={settings.allowFrontendLogin}
                onChange={(ev) => setAllowFrontendLogin(ev.target.checked)}
              />
            </Form.Field>
            <SaveButton isForm saveAction={doSave}>
              Save
            </SaveButton>
            {error != null && <span className="error">{error}</span>}
          </Form.Root>
        </>
      )}
    </>
  );
};

export default Settings;
