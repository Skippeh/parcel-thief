import { ApiResponse, callApi } from ".";
import {
  Baggage,
  Language,
  LocalizedBaggageData,
  QpidArea,
  QpidObject,
} from "../api_types";

export interface QpidAreaWithChildren extends QpidArea {
  sharedCargo: Baggage[];
  garage: QpidObject[];
}

export function getQpidAreas(): Promise<ApiResponse<QpidArea[]>> {
  return callApi<QpidArea[]>("gameData/qpidAreas", "GET");
}

export function getLostBaggages(
  language: Language
): Promise<ApiResponse<Record<number, LocalizedBaggageData[]>>> {
  return callApi(
    `gameData/lostBaggages?lang=${encodeURIComponent(language)}`,
    "GET"
  );
}

/**
 * Organises garage vehicles and shared cargo into the correct areas.
 *
 * Note that none of the input parameters are modified, but the returned areas are only shallow copies, so any changes to the fields of the original or returned areas will affect both.
 * @param areas
 * @param objects
 * @param cargos
 * @returns An array of the organized areas, and the remaining objects and cargos that are not contained in an area's garage or shared cargo inventory.
 */
export function organizeQpidAreas(
  areas: QpidArea[],
  objects: QpidObject[],
  cargos: Baggage[]
): [QpidAreaWithChildren[], QpidObject[], Baggage[]] {
  const areasWithChilden: QpidAreaWithChildren[] = [];
  const newObjects = [...objects];
  const newCargos = [...cargos];

  for (let i = areas.length - 1; i >= 0; --i) {
    const area = areas[i];
    const sharedCargo: Baggage[] = [];
    const garage: QpidObject[] = [];

    for (let j = newCargos.length - 1; j >= 0; --j) {
      const cargo = newCargos[j];

      // Skip cargo not in this area
      if (cargo.locationId != area.qpidId) {
        continue;
      }

      // add to shared cargo if:
      // - cargo position matches qpid area position
      // - cargo xy position is 0,0
      if (
        cargo.location[0] == area.metadata.location[0] ||
        cargo.location[1] == area.metadata.location[1] ||
        cargo.location[0] == 0 ||
        cargo.location[1] == 0
      ) {
        sharedCargo.push(cargo);
        newCargos.splice(j, 1);
      }
    }

    for (let j = newObjects.length - 1; j >= 0; --j) {
      const object = newObjects[j];

      // skip objects not in this area
      if (object.locationId != area.qpidId) {
        continue;
      }

      // add to garage if:
      // - object position matches qpid area position
      // - object xy position is 0,0
      // - isLost is false
      if (
        (object.location[0] == area.metadata.location[0] &&
          object.location[1] == area.metadata.location[1]) ||
        (object.location[0] == 0 && object.location[1] == 0) ||
        !object.isLost
      ) {
        garage.push(object);
        newObjects.splice(j, 1);
      }
    }

    let newArea: QpidAreaWithChildren = {
      ...area,
      sharedCargo,
      garage,
    };

    areasWithChilden.push(newArea);
  }

  return [areasWithChilden, newObjects, newCargos];
}
