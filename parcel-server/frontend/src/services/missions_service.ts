import { callApi } from ".";
import { EditMissionData } from "../api_types";

export async function createMission(
  mission: EditMissionData
): Promise<unknown> {
  return callApi("missions", "POST", mission);
}

export async function getMissions(): Promise<unknown> {
  return callApi("missions", "GET");
}
