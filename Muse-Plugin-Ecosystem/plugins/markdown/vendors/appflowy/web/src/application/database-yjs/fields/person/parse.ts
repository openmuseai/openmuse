import { YDatabaseField, YjsDatabaseKey } from "@/application/types";

import { getTypeOptions } from "../type_option";

import { PersonCellData, PersonTypeOption } from "./person.type";

export function parsePersonTypeOptions(field: YDatabaseField) {
  const content = getTypeOptions(field)?.get(YjsDatabaseKey.content);

  if (!content)
    return {
      persons: [],
    };

  try {
    return JSON.parse(content) as PersonTypeOption;
  } catch (e) {
    return {
      persons: [],
    };
  }
}

export function parsePersonCellData(field: YDatabaseField, data: string): PersonCellData | null {
  const users = parsePersonTypeOptions(field)?.persons;

  if (!data) return null;

  try {
    const userIds = JSON.parse(data);

    return { users: users.filter((user) => userIds.includes(user.id)) };
  } catch (e) {
    return null;
  }
}
