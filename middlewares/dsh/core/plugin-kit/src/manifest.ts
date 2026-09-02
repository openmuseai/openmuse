import { MusePluginCompatibilityError } from "./errors.js";
import type { MusePluginDefinition } from "./types.js";

const PLUGIN_ID = /^[a-z0-9]+(?:[._-][a-z0-9]+)*$/u;
const TOOL_NAME = /^[a-z][a-z0-9_]{0,63}$/u;
const SEMVER = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/u;

const fail = (code: string, message: string): never => {
  throw new MusePluginCompatibilityError(code, message);
};

const nonBlank = (value: string, field: string): void => {
  if (value.length === 0 || value.trim() !== value) fail("INVALID_PLUGIN_MANIFEST", `${field} must be non-blank and trimmed`);
};

const uniqueStrings = (values: readonly string[], field: string): Set<string> => {
  const result = new Set<string>();
  for (const value of values) {
    nonBlank(value, field);
    if (result.has(value)) fail("INVALID_PLUGIN_MANIFEST", `${field} contains duplicate ${value}`);
    result.add(value);
  }
  return result;
};

/**
 * Validate the executable manifest before Cordis owns any effects. Provider compatibility is
 * checked later, but malformed Plugin identity and Tool declarations must never half-activate.
 */
export const validateMusePluginDefinition = (definition: MusePluginDefinition): void => {
  if (!PLUGIN_ID.test(definition.pluginId)) {
    fail("INVALID_PLUGIN_MANIFEST", "pluginId must be a lowercase dotted, dashed, or underscored identifier");
  }
  if (!SEMVER.test(definition.version)) fail("INVALID_PLUGIN_MANIFEST", "version must be strict SemVer");
  if (definition.bridgeMajor !== 1) {
    fail("UNSUPPORTED_BRIDGE_MAJOR", "Plugin requires an unsupported Bridge major");
  }
  if (definition.targets.length === 0) fail("INVALID_PLUGIN_MANIFEST", "Plugin must declare at least one capability target");

  const targetKeys = new Set<string>();
  const toolNames = new Set<string>();
  for (const [targetIndex, target] of definition.targets.entries()) {
    nonBlank(target.familyId, `targets[${targetIndex}].familyId`);
    const range = target.contract;
    if (![range.major, range.minMinor, range.maxMinor].every(Number.isSafeInteger)
      || range.major < 0 || range.minMinor < 0 || range.maxMinor < range.minMinor) {
      fail("INVALID_PLUGIN_MANIFEST", `targets[${targetIndex}].contract is not a valid closed version range`);
    }
    const targetKey = `${target.familyId}@${range.major}`;
    if (targetKeys.has(targetKey)) fail("INVALID_PLUGIN_MANIFEST", `duplicate target ${targetKey}`);
    targetKeys.add(targetKey);

    const required = uniqueStrings(target.requiredOperations, `targets[${targetIndex}].requiredOperations`);
    const optional = uniqueStrings(target.optionalOperations ?? [], `targets[${targetIndex}].optionalOperations`);
    for (const operationId of optional) {
      if (required.has(operationId)) fail("INVALID_PLUGIN_MANIFEST", `operation ${operationId} is both required and optional`);
    }
    if (target.tools.length === 0 && target.optional !== true) {
      fail("INVALID_PLUGIN_MANIFEST", `required target ${target.familyId} contributes no Tools`);
    }
    for (const [toolIndex, tool] of target.tools.entries()) {
      if (!TOOL_NAME.test(tool.name)) {
        fail("INVALID_PLUGIN_MANIFEST", `targets[${targetIndex}].tools[${toolIndex}].name is not a valid DSH Tool name`);
      }
      if (toolNames.has(tool.name)) fail("DUPLICATE_TOOL_NAME", `duplicate Tool name ${tool.name}`);
      toolNames.add(tool.name);
      nonBlank(tool.description, `Tool ${tool.name} description`);
      nonBlank(tool.operationId, `Tool ${tool.name} operationId`);
      if (!required.has(tool.operationId) && !optional.has(tool.operationId)) {
        fail("INVALID_PLUGIN_MANIFEST", `Tool ${tool.name} references undeclared operation ${tool.operationId}`);
      }
      if (tool.timeoutMs !== undefined && (!Number.isSafeInteger(tool.timeoutMs) || tool.timeoutMs <= 0)) {
        fail("INVALID_PLUGIN_MANIFEST", `Tool ${tool.name} timeoutMs must be a positive integer`);
      }
    }
  }
};
