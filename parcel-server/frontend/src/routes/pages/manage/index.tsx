import * as React from "react";
import Code from "../../../components/code";
import * as Form from "../../../components/form";
import * as Tabs from "../../../components/tabs";
import SaveButton from "../../../components/save_button";
import { ApiResponse } from "../../../services";
import { SettingsValues, WhitelistEntry } from "../../../api_types";
import {
  getServerSettings,
  getWhitelist,
  setServerSettings as setSettingsService,
  setWhitelist as setWhitelistService,
} from "../../../services/settings_service";
import WhitelistTable from "./whitelist_table";

const Settings = () => {
  const [settingsError, setSettingsError] = React.useState<string | null>(null);
  const [whitelistError, setWhitelistError] = React.useState<string | null>(
    null
  );
  const [settings, setSettings] = React.useState<
    SettingsValues | undefined | null
  >(undefined);
  const [whitelist, setWhitelist] = React.useState<
    WhitelistEntry[] | undefined | null
  >(undefined);

  async function saveSettings(): Promise<ApiResponse<SettingsValues>> {
    setSettingsError(null);
    const response = await setSettingsService(settings);

    if (response.data != null) {
      setSettings(response.data);
    } else {
      setSettingsError(response.error);
    }

    return response;
  }

  async function saveWhitelist(): Promise<ApiResponse<WhitelistEntry[]>> {
    setWhitelistError(null);
    const response = await setWhitelistService(whitelist);

    if (response.data != null) {
      setWhitelist(response.data);
    } else {
      setWhitelistError(response.error);
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
          setSettingsError(response.error);
        }
      }

      if (whitelist === undefined) {
        const response = await getWhitelist();

        if (response.data != null) {
          setWhitelist(response.data);
        } else {
          setWhitelist(null);
          setSettingsError(response.error);
        }
      }
    })();
  }, []);

  return (
    <>
      {settings === undefined || whitelist === undefined ? (
        "Loading..."
      ) : (
        <Tabs.Root defaultValue="settings">
          <Tabs.List>
            <Tabs.Trigger value="settings">Settings</Tabs.Trigger>
            <Tabs.Trigger value="whitelist">Whitelist</Tabs.Trigger>
          </Tabs.List>
          <Tabs.Content value="settings" $padded>
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
                    The whitelist can be edited from the Whitelist tab above or
                    by changing <Code>data/whitelist.txt</Code> in the server
                    directory.
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
                  If checked, any user with an existing game account can log in
                  to the frontend. Otherwise an admin must first create a
                  frontend account for the user.
                </Form.SubLabel>
                <Form.Control
                  type="checkbox"
                  checked={settings.allowFrontendLogin}
                  onChange={(ev) => setAllowFrontendLogin(ev.target.checked)}
                />
              </Form.Field>
              <SaveButton isForm saveAction={saveSettings}>
                Save
              </SaveButton>
              {settingsError != null && (
                <span className="error">{settingsError}</span>
              )}
            </Form.Root>
          </Tabs.Content>
          <Tabs.Content value="whitelist" $padded>
            <div>
              {settings.publicServer && (
                <span className="warning">
                  Warning: the whitelist is not enforced due to the 'public
                  server' setting being enabled.
                </span>
              )}
              <p>
                The provider id can be found by using the following methods:
              </p>
              <ul>
                <li>
                  Steam: Use{" "}
                  <a href="https://steamid.io" target="_blank">
                    steamid.io
                  </a>{" "}
                  and use the steamID64 value
                </li>
                <li>
                  Epic: Open the Epic Games Launcher and click your avatar, and
                  then click on <Code>Account</Code>. Then look for{" "}
                  <Code>ID</Code> under <Code>Account Information</Code> and use
                  that value
                </li>
              </ul>
              <WhitelistTable
                whitelist={whitelist}
                setWhitelist={setWhitelist}
              />
            </div>
            <SaveButton saveAction={saveWhitelist}>Save</SaveButton>
            {whitelistError != null && (
              <span className="error">{whitelistError}</span>
            )}
          </Tabs.Content>
        </Tabs.Root>
      )}
    </>
  );
};

export default Settings;
