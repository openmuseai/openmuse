#![allow(dead_code, non_snake_case, non_upper_case_globals)]
use std::collections::HashMap;
use std::sync::OnceLock;
// Java: import cn.wps.kfc.xpxml.reader.XmlName2Id
// Java: import cn.wps.kfc.xsd.StringIntMap

// Java field: String URI
pub(crate) const URI_str: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
// Java field: String PREFIX
pub(crate) const PREFIX_str: &str = "";
// Java field: String filterColumn
pub(crate) const filterColumn_str: &str = "filterColumn";
// Java field: String sortState
pub(crate) const sortState_str: &str = "sortState";
// Java field: String extLst
pub(crate) const extLst_str: &str = "extLst";
// Java field: String ref
pub(crate) const r#ref_str: &str = "ref";
// Java field: String filters
pub(crate) const filters_str: &str = "filters";
// Java field: String top10
pub(crate) const top10_str: &str = "top10";
// Java field: String customFilters
pub(crate) const customFilters_str: &str = "customFilters";
// Java field: String dynamicFilter
pub(crate) const dynamicFilter_str: &str = "dynamicFilter";
// Java field: String colorFilter
pub(crate) const colorFilter_str: &str = "colorFilter";
// Java field: String iconFilter
pub(crate) const iconFilter_str: &str = "iconFilter";
// Java field: String colId
pub(crate) const colId_str: &str = "colId";
// Java field: String hiddenButton
pub(crate) const hiddenButton_str: &str = "hiddenButton";
// Java field: String showButton
pub(crate) const showButton_str: &str = "showButton";
// Java field: String filter
pub(crate) const filter_str: &str = "filter";
// Java field: String dateGroupItem
pub(crate) const dateGroupItem_str: &str = "dateGroupItem";
// Java field: String blank
pub(crate) const blank_str: &str = "blank";
// Java field: String calendarType
pub(crate) const calendarType_str: &str = "calendarType";
// Java field: String val
pub(crate) const val_str: &str = "val";
// Java field: String customFilter
pub(crate) const customFilter_str: &str = "customFilter";
// Java field: String and
pub(crate) const and_str: &str = "and";
// Java field: String operator
pub(crate) const operator_str: &str = "operator";
// Java field: String top
pub(crate) const top_str: &str = "top";
// Java field: String percent
pub(crate) const percent_str: &str = "percent";
// Java field: String filterVal
pub(crate) const filterVal_str: &str = "filterVal";
// Java field: String dxfId
pub(crate) const dxfId_str: &str = "dxfId";
// Java field: String cellColor
pub(crate) const cellColor_str: &str = "cellColor";
// Java field: String iconSet
pub(crate) const iconSet_str: &str = "iconSet";
// Java field: String iconId
pub(crate) const iconId_str: &str = "iconId";
// Java field: String type
pub(crate) const r#type_str: &str = "type";
// Java field: String valIso
pub(crate) const valIso_str: &str = "valIso";
// Java field: String maxVal
pub(crate) const maxVal_str: &str = "maxVal";
// Java field: String maxValIso
pub(crate) const maxValIso_str: &str = "maxValIso";
// Java field: String sortCondition
pub(crate) const sortCondition_str: &str = "sortCondition";
// Java field: String columnSort
pub(crate) const columnSort_str: &str = "columnSort";
// Java field: String caseSensitive
pub(crate) const caseSensitive_str: &str = "caseSensitive";
// Java field: String sortMethod
pub(crate) const sortMethod_str: &str = "sortMethod";
// Java field: String descending
pub(crate) const descending_str: &str = "descending";
// Java field: String sortBy
pub(crate) const sortBy_str: &str = "sortBy";
// Java field: String customList
pub(crate) const customList_str: &str = "customList";
// Java field: String year
pub(crate) const year_str: &str = "year";
// Java field: String month
pub(crate) const month_str: &str = "month";
// Java field: String day
pub(crate) const day_str: &str = "day";
// Java field: String hour
pub(crate) const hour_str: &str = "hour";
// Java field: String minute
pub(crate) const minute_str: &str = "minute";
// Java field: String second
pub(crate) const second_str: &str = "second";
// Java field: String dateTimeGrouping
pub(crate) const dateTimeGrouping_str: &str = "dateTimeGrouping";
// Java field: String v
pub(crate) const v_str: &str = "v";
// Java field: String uri
pub(crate) const uri_str: &str = "uri";
// Java field: String moveWithCells
pub(crate) const moveWithCells_str: &str = "moveWithCells";
// Java field: String sizeWithCells
pub(crate) const sizeWithCells_str: &str = "sizeWithCells";
// Java field: String zorder
pub(crate) const zorder_str: &str = "z-order";
// Java field: String ext
pub(crate) const ext_str: &str = "ext";
// Java field: String calcChain
pub(crate) const calcChain_str: &str = "calcChain";
// Java field: String c
pub(crate) const c_str: &str = "c";
// Java field: String r
pub(crate) const r_str: &str = "r";
// Java field: String i
pub(crate) const i_str: &str = "i";
// Java field: String s
pub(crate) const s_str: &str = "s";
// Java field: String l
pub(crate) const l_str: &str = "l";
// Java field: String t
pub(crate) const t_str: &str = "t";
// Java field: String a
pub(crate) const a_str: &str = "a";
// Java field: String comments
pub(crate) const comments_str: &str = "comments";
// Java field: String authors
pub(crate) const authors_str: &str = "authors";
// Java field: String commentList
pub(crate) const commentList_str: &str = "commentList";
// Java field: String author
pub(crate) const author_str: &str = "author";
// Java field: String comment
pub(crate) const comment_str: &str = "comment";
// Java field: String text
pub(crate) const text_str: &str = "text";
// Java field: String commentPr
pub(crate) const commentPr_str: &str = "commentPr";
// Java field: String authorId
pub(crate) const authorId_str: &str = "authorId";
// Java field: String guid
pub(crate) const guid_str: &str = "guid";
// Java field: String shapeId
pub(crate) const shapeId_str: &str = "shapeId";
// Java field: String anchor
pub(crate) const anchor_str: &str = "anchor";
// Java field: String locked
pub(crate) const locked_str: &str = "locked";
// Java field: String defaultSize
pub(crate) const defaultSize_str: &str = "defaultSize";
// Java field: String print
pub(crate) const print_str: &str = "print";
// Java field: String disabled
pub(crate) const disabled_str: &str = "disabled";
// Java field: String uiObject
pub(crate) const uiObject_str: &str = "uiObject";
// Java field: String autoFill
pub(crate) const autoFill_str: &str = "autoFill";
// Java field: String autoLine
pub(crate) const autoLine_str: &str = "autoLine";
// Java field: String altText
pub(crate) const altText_str: &str = "altText";
// Java field: String textHAlign
pub(crate) const textHAlign_str: &str = "textHAlign";
// Java field: String textVAlign
pub(crate) const textVAlign_str: &str = "textVAlign";
// Java field: String lockText
pub(crate) const lockText_str: &str = "lockText";
// Java field: String justLastX
pub(crate) const justLastX_str: &str = "justLastX";
// Java field: String autoScale
pub(crate) const autoScale_str: &str = "autoScale";
// Java field: String rowHidden
pub(crate) const rowHidden_str: &str = "rowHidden";
// Java field: String colHidden
pub(crate) const colHidden_str: &str = "colHidden";
// Java field: String MapInfo
pub(crate) const MapInfo_str: &str = "MapInfo";
// Java field: String Schema
pub(crate) const Schema_str: &str = "Schema";
// Java field: String Map
pub(crate) const Map_str: &str = "Map";
// Java field: String SelectionNamespaces
pub(crate) const SelectionNamespaces_str: &str = "SelectionNamespaces";
// Java field: String ID
pub(crate) const ID_str: &str = "ID";
// Java field: String SchemaRef
pub(crate) const SchemaRef_str: &str = "SchemaRef";
// Java field: String Namespace
pub(crate) const Namespace_str: &str = "Namespace";
// Java field: String SchemaLanguage
pub(crate) const SchemaLanguage_str: &str = "SchemaLanguage";
// Java field: String DataBinding
pub(crate) const DataBinding_str: &str = "DataBinding";
// Java field: String Name
pub(crate) const Name_str: &str = "Name";
// Java field: String RootElement
pub(crate) const RootElement_str: &str = "RootElement";
// Java field: String SchemaID
pub(crate) const SchemaID_str: &str = "SchemaID";
// Java field: String ShowImportExportValidationErrors
pub(crate) const ShowImportExportValidationErrors_str: &str = "ShowImportExportValidationErrors";
// Java field: String AutoFit
pub(crate) const AutoFit_str: &str = "AutoFit";
// Java field: String Append
pub(crate) const Append_str: &str = "Append";
// Java field: String PreserveSortAFLayout
pub(crate) const PreserveSortAFLayout_str: &str = "PreserveSortAFLayout";
// Java field: String PreserveFormat
pub(crate) const PreserveFormat_str: &str = "PreserveFormat";
// Java field: String DataBindingName
pub(crate) const DataBindingName_str: &str = "DataBindingName";
// Java field: String FileBinding
pub(crate) const FileBinding_str: &str = "FileBinding";
// Java field: String ConnectionID
pub(crate) const ConnectionID_str: &str = "ConnectionID";
// Java field: String FileBindingName
pub(crate) const FileBindingName_str: &str = "FileBindingName";
// Java field: String DataBindingLoadMode
pub(crate) const DataBindingLoadMode_str: &str = "DataBindingLoadMode";
// Java field: String connections
pub(crate) const connections_str: &str = "connections";
// Java field: String connection
pub(crate) const connection_str: &str = "connection";
// Java field: String dbPr
pub(crate) const dbPr_str: &str = "dbPr";
// Java field: String olapPr
pub(crate) const olapPr_str: &str = "olapPr";
// Java field: String webPr
pub(crate) const webPr_str: &str = "webPr";
// Java field: String textPr
pub(crate) const textPr_str: &str = "textPr";
// Java field: String parameters
pub(crate) const parameters_str: &str = "parameters";
// Java field: String id
pub(crate) const id_str: &str = "id";
// Java field: String sourceFile
pub(crate) const sourceFile_str: &str = "sourceFile";
// Java field: String odcFile
pub(crate) const odcFile_str: &str = "odcFile";
// Java field: String keepAlive
pub(crate) const keepAlive_str: &str = "keepAlive";
// Java field: String interval
pub(crate) const interval_str: &str = "interval";
// Java field: String name
pub(crate) const name_str: &str = "name";
// Java field: String description
pub(crate) const description_str: &str = "description";
// Java field: String reconnectionMethod
pub(crate) const reconnectionMethod_str: &str = "reconnectionMethod";
// Java field: String refreshedVersion
pub(crate) const refreshedVersion_str: &str = "refreshedVersion";
// Java field: String minRefreshableVersion
pub(crate) const minRefreshableVersion_str: &str = "minRefreshableVersion";
// Java field: String savePassword
pub(crate) const savePassword_str: &str = "savePassword";
// Java field: String new2
pub(crate) const new2_str: &str = "new";
// Java field: String deleted
pub(crate) const deleted_str: &str = "deleted";
// Java field: String onlyUseConnectionFile
pub(crate) const onlyUseConnectionFile_str: &str = "onlyUseConnectionFile";
// Java field: String background
pub(crate) const background_str: &str = "background";
// Java field: String refreshOnLoad
pub(crate) const refreshOnLoad_str: &str = "refreshOnLoad";
// Java field: String saveData
pub(crate) const saveData_str: &str = "saveData";
// Java field: String credentials
pub(crate) const credentials_str: &str = "credentials";
// Java field: String singleSignOnId
pub(crate) const singleSignOnId_str: &str = "singleSignOnId";
// Java field: String command
pub(crate) const command_str: &str = "command";
// Java field: String serverCommand
pub(crate) const serverCommand_str: &str = "serverCommand";
// Java field: String commandType
pub(crate) const commandType_str: &str = "commandType";
// Java field: String local
pub(crate) const local_str: &str = "local";
// Java field: String localConnection
pub(crate) const localConnection_str: &str = "localConnection";
// Java field: String localRefresh
pub(crate) const localRefresh_str: &str = "localRefresh";
// Java field: String sendLocale
pub(crate) const sendLocale_str: &str = "sendLocale";
// Java field: String rowDrillCount
pub(crate) const rowDrillCount_str: &str = "rowDrillCount";
// Java field: String serverFill
pub(crate) const serverFill_str: &str = "serverFill";
// Java field: String serverNumberFormat
pub(crate) const serverNumberFormat_str: &str = "serverNumberFormat";
// Java field: String serverFont
pub(crate) const serverFont_str: &str = "serverFont";
// Java field: String serverFontColor
pub(crate) const serverFontColor_str: &str = "serverFontColor";
// Java field: String tables
pub(crate) const tables_str: &str = "tables";
// Java field: String xml
pub(crate) const xml_str: &str = "xml";
// Java field: String sourceData
pub(crate) const sourceData_str: &str = "sourceData";
// Java field: String parsePre
pub(crate) const parsePre_str: &str = "parsePre";
// Java field: String consecutive
pub(crate) const consecutive_str: &str = "consecutive";
// Java field: String firstRow
pub(crate) const firstRow_str: &str = "firstRow";
// Java field: String xl97
pub(crate) const xl97_str: &str = "xl97";
// Java field: String textDates
pub(crate) const textDates_str: &str = "textDates";
// Java field: String xl2000
pub(crate) const xl2000_str: &str = "xl2000";
// Java field: String url
pub(crate) const url_str: &str = "url";
// Java field: String post
pub(crate) const post_str: &str = "post";
// Java field: String htmlTables
pub(crate) const htmlTables_str: &str = "htmlTables";
// Java field: String htmlFormat
pub(crate) const htmlFormat_str: &str = "htmlFormat";
// Java field: String editPage
pub(crate) const editPage_str: &str = "editPage";
// Java field: String parameter
pub(crate) const parameter_str: &str = "parameter";
// Java field: String count
pub(crate) const count_str: &str = "count";
// Java field: String sqlType
pub(crate) const sqlType_str: &str = "sqlType";
// Java field: String parameterType
pub(crate) const parameterType_str: &str = "parameterType";
// Java field: String refreshOnChange
pub(crate) const refreshOnChange_str: &str = "refreshOnChange";
// Java field: String prompt
pub(crate) const prompt_str: &str = "prompt";
// Java field: String boolean2
pub(crate) const boolean2_str: &str = "boolean";
// Java field: String double2
pub(crate) const double2_str: &str = "double";
// Java field: String integer
pub(crate) const integer_str: &str = "integer";
// Java field: String string
pub(crate) const string_str: &str = "string";
// Java field: String cell
pub(crate) const cell_str: &str = "cell";
// Java field: String m
pub(crate) const m_str: &str = "m";
// Java field: String x
pub(crate) const x_str: &str = "x";
// Java field: String textFields
pub(crate) const textFields_str: &str = "textFields";
// Java field: String fileType
pub(crate) const fileType_str: &str = "fileType";
// Java field: String codePage
pub(crate) const codePage_str: &str = "codePage";
// Java field: String characterSet
pub(crate) const characterSet_str: &str = "characterSet";
// Java field: String delimited
pub(crate) const delimited_str: &str = "delimited";
// Java field: String decimal
pub(crate) const decimal_str: &str = "decimal";
// Java field: String thousands
pub(crate) const thousands_str: &str = "thousands";
// Java field: String tab
pub(crate) const tab_str: &str = "tab";
// Java field: String space
pub(crate) const space_str: &str = "space";
// Java field: String comma
pub(crate) const comma_str: &str = "comma";
// Java field: String semicolon
pub(crate) const semicolon_str: &str = "semicolon";
// Java field: String qualifier
pub(crate) const qualifier_str: &str = "qualifier";
// Java field: String delimiter
pub(crate) const delimiter_str: &str = "delimiter";
// Java field: String textField
pub(crate) const textField_str: &str = "textField";
// Java field: String position
pub(crate) const position_str: &str = "position";
// Java field: String pivotCacheDefinition
pub(crate) const pivotCacheDefinition_str: &str = "pivotCacheDefinition";
// Java field: String pivotCacheRecords
pub(crate) const pivotCacheRecords_str: &str = "pivotCacheRecords";
// Java field: String pivotTableDefinition
pub(crate) const pivotTableDefinition_str: &str = "pivotTableDefinition";
// Java field: String cacheSource
pub(crate) const cacheSource_str: &str = "cacheSource";
// Java field: String cacheFields
pub(crate) const cacheFields_str: &str = "cacheFields";
// Java field: String cacheHierarchies
pub(crate) const cacheHierarchies_str: &str = "cacheHierarchies";
// Java field: String kpis
pub(crate) const kpis_str: &str = "kpis";
// Java field: String tupleCache
pub(crate) const tupleCache_str: &str = "tupleCache";
// Java field: String calculatedItems
pub(crate) const calculatedItems_str: &str = "calculatedItems";
// Java field: String calculatedMembers
pub(crate) const calculatedMembers_str: &str = "calculatedMembers";
// Java field: String dimensions
pub(crate) const dimensions_str: &str = "dimensions";
// Java field: String measureGroups
pub(crate) const measureGroups_str: &str = "measureGroups";
// Java field: String maps
pub(crate) const maps_str: &str = "maps";
// Java field: String invalid
pub(crate) const invalid_str: &str = "invalid";
// Java field: String optimizeMemory
pub(crate) const optimizeMemory_str: &str = "optimizeMemory";
// Java field: String enableRefresh
pub(crate) const enableRefresh_str: &str = "enableRefresh";
// Java field: String refreshedBy
pub(crate) const refreshedBy_str: &str = "refreshedBy";
// Java field: String refreshedDate
pub(crate) const refreshedDate_str: &str = "refreshedDate";
// Java field: String refreshedDateIso
pub(crate) const refreshedDateIso_str: &str = "refreshedDateIso";
// Java field: String backgroundQuery
pub(crate) const backgroundQuery_str: &str = "backgroundQuery";
// Java field: String missingItemsLimit
pub(crate) const missingItemsLimit_str: &str = "missingItemsLimit";
// Java field: String createdVersion
pub(crate) const createdVersion_str: &str = "createdVersion";
// Java field: String recordCount
pub(crate) const recordCount_str: &str = "recordCount";
// Java field: String upgradeOnRefresh
pub(crate) const upgradeOnRefresh_str: &str = "upgradeOnRefresh";
// Java field: String supportSubquery
pub(crate) const supportSubquery_str: &str = "supportSubquery";
// Java field: String supportAdvancedDrill
pub(crate) const supportAdvancedDrill_str: &str = "supportAdvancedDrill";
// Java field: String cacheField
pub(crate) const cacheField_str: &str = "cacheField";
// Java field: String sharedItems
pub(crate) const sharedItems_str: &str = "sharedItems";
// Java field: String fieldGroup
pub(crate) const fieldGroup_str: &str = "fieldGroup";
// Java field: String mpMap
pub(crate) const mpMap_str: &str = "mpMap";
// Java field: String caption
pub(crate) const caption_str: &str = "caption";
// Java field: String propertyName
pub(crate) const propertyName_str: &str = "propertyName";
// Java field: String serverField
pub(crate) const serverField_str: &str = "serverField";
// Java field: String uniqueList
pub(crate) const uniqueList_str: &str = "uniqueList";
// Java field: String numFmtId
pub(crate) const numFmtId_str: &str = "numFmtId";
// Java field: String formula
pub(crate) const formula_str: &str = "formula";
// Java field: String hierarchy
pub(crate) const hierarchy_str: &str = "hierarchy";
// Java field: String level
pub(crate) const level_str: &str = "level";
// Java field: String databaseField
pub(crate) const databaseField_str: &str = "databaseField";
// Java field: String mappingCount
pub(crate) const mappingCount_str: &str = "mappingCount";
// Java field: String memberPropertyField
pub(crate) const memberPropertyField_str: &str = "memberPropertyField";
// Java field: String worksheetSource
pub(crate) const worksheetSource_str: &str = "worksheetSource";
// Java field: String consolidation
pub(crate) const consolidation_str: &str = "consolidation";
// Java field: String connectionId
pub(crate) const connectionId_str: &str = "connectionId";
// Java field: String sheet
pub(crate) const sheet_str: &str = "sheet";
// Java field: String pages
pub(crate) const pages_str: &str = "pages";
// Java field: String rangeSets
pub(crate) const rangeSets_str: &str = "rangeSets";
// Java field: String autoPage
pub(crate) const autoPage_str: &str = "autoPage";
// Java field: String page
pub(crate) const page_str: &str = "page";
// Java field: String pageItem
pub(crate) const pageItem_str: &str = "pageItem";
// Java field: String rangeSet
pub(crate) const rangeSet_str: &str = "rangeSet";
// Java field: String i1
pub(crate) const i1_str: &str = "i1";
// Java field: String i2
pub(crate) const i2_str: &str = "i2";
// Java field: String i3
pub(crate) const i3_str: &str = "i3";
// Java field: String i4
pub(crate) const i4_str: &str = "i4";
// Java field: String n
pub(crate) const n_str: &str = "n";
// Java field: String b
pub(crate) const b_str: &str = "b";
// Java field: String e
pub(crate) const e_str: &str = "e";
// Java field: String d
pub(crate) const d_str: &str = "d";
// Java field: String containsSemiMixedTypes
pub(crate) const containsSemiMixedTypes_str: &str = "containsSemiMixedTypes";
// Java field: String containsNonDate
pub(crate) const containsNonDate_str: &str = "containsNonDate";
// Java field: String containsDate
pub(crate) const containsDate_str: &str = "containsDate";
// Java field: String containsString
pub(crate) const containsString_str: &str = "containsString";
// Java field: String containsBlank
pub(crate) const containsBlank_str: &str = "containsBlank";
// Java field: String containsMixedTypes
pub(crate) const containsMixedTypes_str: &str = "containsMixedTypes";
// Java field: String containsNumber
pub(crate) const containsNumber_str: &str = "containsNumber";
// Java field: String containsInteger
pub(crate) const containsInteger_str: &str = "containsInteger";
// Java field: String minValue
pub(crate) const minValue_str: &str = "minValue";
// Java field: String maxValue
pub(crate) const maxValue_str: &str = "maxValue";
// Java field: String minDate
pub(crate) const minDate_str: &str = "minDate";
// Java field: String maxDate
pub(crate) const maxDate_str: &str = "maxDate";
// Java field: String longText
pub(crate) const longText_str: &str = "longText";
// Java field: String tpls
pub(crate) const tpls_str: &str = "tpls";
// Java field: String u
pub(crate) const u_str: &str = "u";
// Java field: String f
pub(crate) const f_str: &str = "f";
// Java field: String cp
pub(crate) const cp_str: &str = "cp";
// Java field: String in
pub(crate) const r#in_str: &str = "in";
// Java field: String bc
pub(crate) const bc_str: &str = "bc";
// Java field: String fc
pub(crate) const fc_str: &str = "fc";
// Java field: String un
pub(crate) const un_str: &str = "un";
// Java field: String st
pub(crate) const st_str: &str = "st";
// Java field: String rangePr
pub(crate) const rangePr_str: &str = "rangePr";
// Java field: String discretePr
pub(crate) const discretePr_str: &str = "discretePr";
// Java field: String groupItems
pub(crate) const groupItems_str: &str = "groupItems";
// Java field: String par
pub(crate) const par_str: &str = "par";
// Java field: String base
pub(crate) const base_str: &str = "base";
// Java field: String autoStart
pub(crate) const autoStart_str: &str = "autoStart";
// Java field: String autoEnd
pub(crate) const autoEnd_str: &str = "autoEnd";
// Java field: String groupBy
pub(crate) const groupBy_str: &str = "groupBy";
// Java field: String startNum
pub(crate) const startNum_str: &str = "startNum";
// Java field: String endNum
pub(crate) const endNum_str: &str = "endNum";
// Java field: String startDate
pub(crate) const startDate_str: &str = "startDate";
// Java field: String endDate
pub(crate) const endDate_str: &str = "endDate";
// Java field: String groupInterval
pub(crate) const groupInterval_str: &str = "groupInterval";
// Java field: String kpi
pub(crate) const kpi_str: &str = "kpi";
// Java field: String uniqueName
pub(crate) const uniqueName_str: &str = "uniqueName";
// Java field: String displayFolder
pub(crate) const displayFolder_str: &str = "displayFolder";
// Java field: String measureGroup
pub(crate) const measureGroup_str: &str = "measureGroup";
// Java field: String parent
pub(crate) const parent_str: &str = "parent";
// Java field: String value
pub(crate) const value_str: &str = "value";
// Java field: String goal
pub(crate) const goal_str: &str = "goal";
// Java field: String status
pub(crate) const status_str: &str = "status";
// Java field: String trend
pub(crate) const trend_str: &str = "trend";
// Java field: String weight
pub(crate) const weight_str: &str = "weight";
// Java field: String time
pub(crate) const time_str: &str = "time";
// Java field: String cacheHierarchy
pub(crate) const cacheHierarchy_str: &str = "cacheHierarchy";
// Java field: String fieldsUsage
pub(crate) const fieldsUsage_str: &str = "fieldsUsage";
// Java field: String groupLevels
pub(crate) const groupLevels_str: &str = "groupLevels";
// Java field: String measure
pub(crate) const measure_str: &str = "measure";
// Java field: String set
pub(crate) const set_str: &str = "set";
// Java field: String parentSet
pub(crate) const parentSet_str: &str = "parentSet";
// Java field: String attribute
pub(crate) const attribute_str: &str = "attribute";
// Java field: String keyAttribute
pub(crate) const keyAttribute_str: &str = "keyAttribute";
// Java field: String defaultMemberUniqueName
pub(crate) const defaultMemberUniqueName_str: &str = "defaultMemberUniqueName";
// Java field: String allUniqueName
pub(crate) const allUniqueName_str: &str = "allUniqueName";
// Java field: String allCaption
pub(crate) const allCaption_str: &str = "allCaption";
// Java field: String dimensionUniqueName
pub(crate) const dimensionUniqueName_str: &str = "dimensionUniqueName";
// Java field: String measures
pub(crate) const measures_str: &str = "measures";
// Java field: String oneField
pub(crate) const oneField_str: &str = "oneField";
// Java field: String memberValueDatatype
pub(crate) const memberValueDatatype_str: &str = "memberValueDatatype";
// Java field: String unbalanced
pub(crate) const unbalanced_str: &str = "unbalanced";
// Java field: String unbalancedGroup
pub(crate) const unbalancedGroup_str: &str = "unbalancedGroup";
// Java field: String hidden
pub(crate) const hidden_str: &str = "hidden";
// Java field: String fieldUsage
pub(crate) const fieldUsage_str: &str = "fieldUsage";
// Java field: String groupLevel
pub(crate) const groupLevel_str: &str = "groupLevel";
// Java field: String groups
pub(crate) const groups_str: &str = "groups";
// Java field: String user
pub(crate) const user_str: &str = "user";
// Java field: String customRollUp
pub(crate) const customRollUp_str: &str = "customRollUp";
// Java field: String group
pub(crate) const group_str: &str = "group";
// Java field: String groupMembers
pub(crate) const groupMembers_str: &str = "groupMembers";
// Java field: String uniqueParent
pub(crate) const uniqueParent_str: &str = "uniqueParent";
// Java field: String groupMember
pub(crate) const groupMember_str: &str = "groupMember";
// Java field: String entries
pub(crate) const entries_str: &str = "entries";
// Java field: String sets
pub(crate) const sets_str: &str = "sets";
// Java field: String queryCache
pub(crate) const queryCache_str: &str = "queryCache";
// Java field: String serverFormats
pub(crate) const serverFormats_str: &str = "serverFormats";
// Java field: String culture
pub(crate) const culture_str: &str = "culture";
// Java field: String format
pub(crate) const format_str: &str = "format";
// Java field: String serverFormat
pub(crate) const serverFormat_str: &str = "serverFormat";
// Java field: String tpl
pub(crate) const tpl_str: &str = "tpl";
// Java field: String fld
pub(crate) const fld_str: &str = "fld";
// Java field: String hier
pub(crate) const hier_str: &str = "hier";
// Java field: String item
pub(crate) const item_str: &str = "item";
// Java field: String sortByTuple
pub(crate) const sortByTuple_str: &str = "sortByTuple";
// Java field: String maxRank
pub(crate) const maxRank_str: &str = "maxRank";
// Java field: String setDefinition
pub(crate) const setDefinition_str: &str = "setDefinition";
// Java field: String sortType
pub(crate) const sortType_str: &str = "sortType";
// Java field: String queryFailed
pub(crate) const queryFailed_str: &str = "queryFailed";
// Java field: String query
pub(crate) const query_str: &str = "query";
// Java field: String mdx
pub(crate) const mdx_str: &str = "mdx";
// Java field: String calculatedItem
pub(crate) const calculatedItem_str: &str = "calculatedItem";
// Java field: String pivotArea
pub(crate) const pivotArea_str: &str = "pivotArea";
// Java field: String field
pub(crate) const field_str: &str = "field";
// Java field: String calculatedMember
pub(crate) const calculatedMember_str: &str = "calculatedMember";
// Java field: String memberName
pub(crate) const memberName_str: &str = "memberName";
// Java field: String solveOrder
pub(crate) const solveOrder_str: &str = "solveOrder";
// Java field: String location
pub(crate) const location_str: &str = "location";
// Java field: String pivotFields
pub(crate) const pivotFields_str: &str = "pivotFields";
// Java field: String rowFields
pub(crate) const rowFields_str: &str = "rowFields";
// Java field: String rowItems
pub(crate) const rowItems_str: &str = "rowItems";
// Java field: String colFields
pub(crate) const colFields_str: &str = "colFields";
// Java field: String colItems
pub(crate) const colItems_str: &str = "colItems";
// Java field: String pageFields
pub(crate) const pageFields_str: &str = "pageFields";
// Java field: String dataFields
pub(crate) const dataFields_str: &str = "dataFields";
// Java field: String formats
pub(crate) const formats_str: &str = "formats";
// Java field: String conditionalFormats
pub(crate) const conditionalFormats_str: &str = "conditionalFormats";
// Java field: String chartFormats
pub(crate) const chartFormats_str: &str = "chartFormats";
// Java field: String pivotHierarchies
pub(crate) const pivotHierarchies_str: &str = "pivotHierarchies";
// Java field: String pivotTableStyleInfo
pub(crate) const pivotTableStyleInfo_str: &str = "pivotTableStyleInfo";
// Java field: String rowHierarchiesUsage
pub(crate) const rowHierarchiesUsage_str: &str = "rowHierarchiesUsage";
// Java field: String colHierarchiesUsage
pub(crate) const colHierarchiesUsage_str: &str = "colHierarchiesUsage";
// Java field: String cacheId
pub(crate) const cacheId_str: &str = "cacheId";
// Java field: String dataOnRows
pub(crate) const dataOnRows_str: &str = "dataOnRows";
// Java field: String dataPosition
pub(crate) const dataPosition_str: &str = "dataPosition";
// Java field: String dataCaption
pub(crate) const dataCaption_str: &str = "dataCaption";
// Java field: String grandTotalCaption
pub(crate) const grandTotalCaption_str: &str = "grandTotalCaption";
// Java field: String errorCaption
pub(crate) const errorCaption_str: &str = "errorCaption";
// Java field: String showError
pub(crate) const showError_str: &str = "showError";
// Java field: String missingCaption
pub(crate) const missingCaption_str: &str = "missingCaption";
// Java field: String showMissing
pub(crate) const showMissing_str: &str = "showMissing";
// Java field: String pageStyle
pub(crate) const pageStyle_str: &str = "pageStyle";
// Java field: String pivotTableStyle
pub(crate) const pivotTableStyle_str: &str = "pivotTableStyle";
// Java field: String vacatedStyle
pub(crate) const vacatedStyle_str: &str = "vacatedStyle";
// Java field: String tag
pub(crate) const tag_str: &str = "tag";
// Java field: String updatedVersion
pub(crate) const updatedVersion_str: &str = "updatedVersion";
// Java field: String asteriskTotals
pub(crate) const asteriskTotals_str: &str = "asteriskTotals";
// Java field: String showItems
pub(crate) const showItems_str: &str = "showItems";
// Java field: String editData
pub(crate) const editData_str: &str = "editData";
// Java field: String disableFieldList
pub(crate) const disableFieldList_str: &str = "disableFieldList";
// Java field: String showCalcMbrs
pub(crate) const showCalcMbrs_str: &str = "showCalcMbrs";
// Java field: String visualTotals
pub(crate) const visualTotals_str: &str = "visualTotals";
// Java field: String showMultipleLabel
pub(crate) const showMultipleLabel_str: &str = "showMultipleLabel";
// Java field: String showDataDropDown
pub(crate) const showDataDropDown_str: &str = "showDataDropDown";
// Java field: String showDrill
pub(crate) const showDrill_str: &str = "showDrill";
// Java field: String printDrill
pub(crate) const printDrill_str: &str = "printDrill";
// Java field: String showMemberPropertyTips
pub(crate) const showMemberPropertyTips_str: &str = "showMemberPropertyTips";
// Java field: String showDataTips
pub(crate) const showDataTips_str: &str = "showDataTips";
// Java field: String enableWizard
pub(crate) const enableWizard_str: &str = "enableWizard";
// Java field: String enableDrill
pub(crate) const enableDrill_str: &str = "enableDrill";
// Java field: String enableFieldProperties
pub(crate) const enableFieldProperties_str: &str = "enableFieldProperties";
// Java field: String preserveFormatting
pub(crate) const preserveFormatting_str: &str = "preserveFormatting";
// Java field: String useAutoFormatting
pub(crate) const useAutoFormatting_str: &str = "useAutoFormatting";
// Java field: String pageWrap
pub(crate) const pageWrap_str: &str = "pageWrap";
// Java field: String pageOverThenDown
pub(crate) const pageOverThenDown_str: &str = "pageOverThenDown";
// Java field: String subtotalHiddenItems
pub(crate) const subtotalHiddenItems_str: &str = "subtotalHiddenItems";
// Java field: String rowGrandTotals
pub(crate) const rowGrandTotals_str: &str = "rowGrandTotals";
// Java field: String colGrandTotals
pub(crate) const colGrandTotals_str: &str = "colGrandTotals";
// Java field: String fieldPrintTitles
pub(crate) const fieldPrintTitles_str: &str = "fieldPrintTitles";
// Java field: String itemPrintTitles
pub(crate) const itemPrintTitles_str: &str = "itemPrintTitles";
// Java field: String mergeItem
pub(crate) const mergeItem_str: &str = "mergeItem";
// Java field: String showDropZones
pub(crate) const showDropZones_str: &str = "showDropZones";
// Java field: String indent
pub(crate) const indent_str: &str = "indent";
// Java field: String showEmptyRow
pub(crate) const showEmptyRow_str: &str = "showEmptyRow";
// Java field: String showEmptyCol
pub(crate) const showEmptyCol_str: &str = "showEmptyCol";
// Java field: String showHeaders
pub(crate) const showHeaders_str: &str = "showHeaders";
// Java field: String compact
pub(crate) const compact_str: &str = "compact";
// Java field: String outline
pub(crate) const outline_str: &str = "outline";
// Java field: String outlineData
pub(crate) const outlineData_str: &str = "outlineData";
// Java field: String compactData
pub(crate) const compactData_str: &str = "compactData";
// Java field: String published
pub(crate) const published_str: &str = "published";
// Java field: String gridDropZones
pub(crate) const gridDropZones_str: &str = "gridDropZones";
// Java field: String immersive
pub(crate) const immersive_str: &str = "immersive";
// Java field: String multipleFieldFilters
pub(crate) const multipleFieldFilters_str: &str = "multipleFieldFilters";
// Java field: String chartFormat
pub(crate) const chartFormat_str: &str = "chartFormat";
// Java field: String rowHeaderCaption
pub(crate) const rowHeaderCaption_str: &str = "rowHeaderCaption";
// Java field: String colHeaderCaption
pub(crate) const colHeaderCaption_str: &str = "colHeaderCaption";
// Java field: String fieldListSortAscending
pub(crate) const fieldListSortAscending_str: &str = "fieldListSortAscending";
// Java field: String mdxSubqueries
pub(crate) const mdxSubqueries_str: &str = "mdxSubqueries";
// Java field: String customListSort
pub(crate) const customListSort_str: &str = "customListSort";
// Java field: String firstHeaderRow
pub(crate) const firstHeaderRow_str: &str = "firstHeaderRow";
// Java field: String firstDataRow
pub(crate) const firstDataRow_str: &str = "firstDataRow";
// Java field: String firstDataCol
pub(crate) const firstDataCol_str: &str = "firstDataCol";
// Java field: String rowPageCount
pub(crate) const rowPageCount_str: &str = "rowPageCount";
// Java field: String colPageCount
pub(crate) const colPageCount_str: &str = "colPageCount";
// Java field: String pivotField
pub(crate) const pivotField_str: &str = "pivotField";
// Java field: String items
pub(crate) const items_str: &str = "items";
// Java field: String autoSortScope
pub(crate) const autoSortScope_str: &str = "autoSortScope";
// Java field: String axis
pub(crate) const axis_str: &str = "axis";
// Java field: String dataField
pub(crate) const dataField_str: &str = "dataField";
// Java field: String subtotalCaption
pub(crate) const subtotalCaption_str: &str = "subtotalCaption";
// Java field: String showDropDowns
pub(crate) const showDropDowns_str: &str = "showDropDowns";
// Java field: String hiddenLevel
pub(crate) const hiddenLevel_str: &str = "hiddenLevel";
// Java field: String uniqueMemberProperty
pub(crate) const uniqueMemberProperty_str: &str = "uniqueMemberProperty";
// Java field: String allDrilled
pub(crate) const allDrilled_str: &str = "allDrilled";
// Java field: String subtotalTop
pub(crate) const subtotalTop_str: &str = "subtotalTop";
// Java field: String dragToRow
pub(crate) const dragToRow_str: &str = "dragToRow";
// Java field: String dragToCol
pub(crate) const dragToCol_str: &str = "dragToCol";
// Java field: String multipleItemSelectionAllowed
pub(crate) const multipleItemSelectionAllowed_str: &str = "multipleItemSelectionAllowed";
// Java field: String dragToPage
pub(crate) const dragToPage_str: &str = "dragToPage";
// Java field: String dragToData
pub(crate) const dragToData_str: &str = "dragToData";
// Java field: String dragOff
pub(crate) const dragOff_str: &str = "dragOff";
// Java field: String showAll
pub(crate) const showAll_str: &str = "showAll";
// Java field: String insertBlankRow
pub(crate) const insertBlankRow_str: &str = "insertBlankRow";
// Java field: String insertPageBreak
pub(crate) const insertPageBreak_str: &str = "insertPageBreak";
// Java field: String autoShow
pub(crate) const autoShow_str: &str = "autoShow";
// Java field: String topAutoShow
pub(crate) const topAutoShow_str: &str = "topAutoShow";
// Java field: String hideNewItems
pub(crate) const hideNewItems_str: &str = "hideNewItems";
// Java field: String measureFilter
pub(crate) const measureFilter_str: &str = "measureFilter";
// Java field: String includeNewItemsInFilter
pub(crate) const includeNewItemsInFilter_str: &str = "includeNewItemsInFilter";
// Java field: String itemPageCount
pub(crate) const itemPageCount_str: &str = "itemPageCount";
// Java field: String dataSourceSort
pub(crate) const dataSourceSort_str: &str = "dataSourceSort";
// Java field: String nonAutoSortDefault
pub(crate) const nonAutoSortDefault_str: &str = "nonAutoSortDefault";
// Java field: String rankBy
pub(crate) const rankBy_str: &str = "rankBy";
// Java field: String defaultSubtotal
pub(crate) const defaultSubtotal_str: &str = "defaultSubtotal";
// Java field: String sumSubtotal
pub(crate) const sumSubtotal_str: &str = "sumSubtotal";
// Java field: String countASubtotal
pub(crate) const countASubtotal_str: &str = "countASubtotal";
// Java field: String avgSubtotal
pub(crate) const avgSubtotal_str: &str = "avgSubtotal";
// Java field: String maxSubtotal
pub(crate) const maxSubtotal_str: &str = "maxSubtotal";
// Java field: String minSubtotal
pub(crate) const minSubtotal_str: &str = "minSubtotal";
// Java field: String productSubtotal
pub(crate) const productSubtotal_str: &str = "productSubtotal";
// Java field: String countSubtotal
pub(crate) const countSubtotal_str: &str = "countSubtotal";
// Java field: String stdDevSubtotal
pub(crate) const stdDevSubtotal_str: &str = "stdDevSubtotal";
// Java field: String stdDevPSubtotal
pub(crate) const stdDevPSubtotal_str: &str = "stdDevPSubtotal";
// Java field: String varSubtotal
pub(crate) const varSubtotal_str: &str = "varSubtotal";
// Java field: String varPSubtotal
pub(crate) const varPSubtotal_str: &str = "varPSubtotal";
// Java field: String showPropCell
pub(crate) const showPropCell_str: &str = "showPropCell";
// Java field: String showPropTip
pub(crate) const showPropTip_str: &str = "showPropTip";
// Java field: String showPropAsCaption
pub(crate) const showPropAsCaption_str: &str = "showPropAsCaption";
// Java field: String defaultAttributeDrillState
pub(crate) const defaultAttributeDrillState_str: &str = "defaultAttributeDrillState";
// Java field: String h
pub(crate) const h_str: &str = "h";
// Java field: String sd
pub(crate) const sd_str: &str = "sd";
// Java field: String pageField
pub(crate) const pageField_str: &str = "pageField";
// Java field: String cap
pub(crate) const cap_str: &str = "cap";
// Java field: String subtotal
pub(crate) const subtotal_str: &str = "subtotal";
// Java field: String showDataAs
pub(crate) const showDataAs_str: &str = "showDataAs";
// Java field: String baseField
pub(crate) const baseField_str: &str = "baseField";
// Java field: String baseItem
pub(crate) const baseItem_str: &str = "baseItem";
// Java field: String action
pub(crate) const action_str: &str = "action";
// Java field: String conditionalFormat
pub(crate) const conditionalFormat_str: &str = "conditionalFormat";
// Java field: String pivotAreas
pub(crate) const pivotAreas_str: &str = "pivotAreas";
// Java field: String scope
pub(crate) const scope_str: &str = "scope";
// Java field: String priority
pub(crate) const priority_str: &str = "priority";
// Java field: String chart
pub(crate) const chart_str: &str = "chart";
// Java field: String series
pub(crate) const series_str: &str = "series";
// Java field: String pivotHierarchy
pub(crate) const pivotHierarchy_str: &str = "pivotHierarchy";
// Java field: String mps
pub(crate) const mps_str: &str = "mps";
// Java field: String members
pub(crate) const members_str: &str = "members";
// Java field: String showInFieldList
pub(crate) const showInFieldList_str: &str = "showInFieldList";
// Java field: String rowHierarchyUsage
pub(crate) const rowHierarchyUsage_str: &str = "rowHierarchyUsage";
// Java field: String colHierarchyUsage
pub(crate) const colHierarchyUsage_str: &str = "colHierarchyUsage";
// Java field: String hierarchyUsage
pub(crate) const hierarchyUsage_str: &str = "hierarchyUsage";
// Java field: String mp
pub(crate) const mp_str: &str = "mp";
// Java field: String showCell
pub(crate) const showCell_str: &str = "showCell";
// Java field: String showTip
pub(crate) const showTip_str: &str = "showTip";
// Java field: String showAsCaption
pub(crate) const showAsCaption_str: &str = "showAsCaption";
// Java field: String nameLen
pub(crate) const nameLen_str: &str = "nameLen";
// Java field: String pPos
pub(crate) const pPos_str: &str = "pPos";
// Java field: String pLen
pub(crate) const pLen_str: &str = "pLen";
// Java field: String member
pub(crate) const member_str: &str = "member";
// Java field: String dimension
pub(crate) const dimension_str: &str = "dimension";
// Java field: String map
pub(crate) const map_str: &str = "map";
// Java field: String showRowHeaders
pub(crate) const showRowHeaders_str: &str = "showRowHeaders";
// Java field: String showColHeaders
pub(crate) const showColHeaders_str: &str = "showColHeaders";
// Java field: String showRowStripes
pub(crate) const showRowStripes_str: &str = "showRowStripes";
// Java field: String showColStripes
pub(crate) const showColStripes_str: &str = "showColStripes";
// Java field: String showLastColumn
pub(crate) const showLastColumn_str: &str = "showLastColumn";
// Java field: String autoFilter
pub(crate) const autoFilter_str: &str = "autoFilter";
// Java field: String mpFld
pub(crate) const mpFld_str: &str = "mpFld";
// Java field: String evalOrder
pub(crate) const evalOrder_str: &str = "evalOrder";
// Java field: String iMeasureHier
pub(crate) const iMeasureHier_str: &str = "iMeasureHier";
// Java field: String iMeasureFld
pub(crate) const iMeasureFld_str: &str = "iMeasureFld";
// Java field: String stringValue1
pub(crate) const stringValue1_str: &str = "stringValue1";
// Java field: String stringValue2
pub(crate) const stringValue2_str: &str = "stringValue2";
// Java field: String references
pub(crate) const references_str: &str = "references";
// Java field: String dataOnly
pub(crate) const dataOnly_str: &str = "dataOnly";
// Java field: String labelOnly
pub(crate) const labelOnly_str: &str = "labelOnly";
// Java field: String grandRow
pub(crate) const grandRow_str: &str = "grandRow";
// Java field: String grandCol
pub(crate) const grandCol_str: &str = "grandCol";
// Java field: String cacheIndex
pub(crate) const cacheIndex_str: &str = "cacheIndex";
// Java field: String offset
pub(crate) const offset_str: &str = "offset";
// Java field: String collapsedLevelsAreSubtotals
pub(crate) const collapsedLevelsAreSubtotals_str: &str = "collapsedLevelsAreSubtotals";
// Java field: String fieldPosition
pub(crate) const fieldPosition_str: &str = "fieldPosition";
// Java field: String reference
pub(crate) const reference_str: &str = "reference";
// Java field: String selected
pub(crate) const selected_str: &str = "selected";
// Java field: String byPosition
pub(crate) const byPosition_str: &str = "byPosition";
// Java field: String relative
pub(crate) const relative_str: &str = "relative";
// Java field: String queryTable
pub(crate) const queryTable_str: &str = "queryTable";
// Java field: String queryTableRefresh
pub(crate) const queryTableRefresh_str: &str = "queryTableRefresh";
// Java field: String headers
pub(crate) const headers_str: &str = "headers";
// Java field: String rowNumbers
pub(crate) const rowNumbers_str: &str = "rowNumbers";
// Java field: String disableRefresh
pub(crate) const disableRefresh_str: &str = "disableRefresh";
// Java field: String backgroundRefresh
pub(crate) const backgroundRefresh_str: &str = "backgroundRefresh";
// Java field: String firstBackgroundRefresh
pub(crate) const firstBackgroundRefresh_str: &str = "firstBackgroundRefresh";
// Java field: String growShrinkType
pub(crate) const growShrinkType_str: &str = "growShrinkType";
// Java field: String fillFormulas
pub(crate) const fillFormulas_str: &str = "fillFormulas";
// Java field: String removeDataOnSave
pub(crate) const removeDataOnSave_str: &str = "removeDataOnSave";
// Java field: String disableEdit
pub(crate) const disableEdit_str: &str = "disableEdit";
// Java field: String adjustColumnWidth
pub(crate) const adjustColumnWidth_str: &str = "adjustColumnWidth";
// Java field: String intermediate
pub(crate) const intermediate_str: &str = "intermediate";
// Java field: String queryTableFields
pub(crate) const queryTableFields_str: &str = "queryTableFields";
// Java field: String queryTableDeletedFields
pub(crate) const queryTableDeletedFields_str: &str = "queryTableDeletedFields";
// Java field: String preserveSortFilterLayout
pub(crate) const preserveSortFilterLayout_str: &str = "preserveSortFilterLayout";
// Java field: String fieldIdWrapped
pub(crate) const fieldIdWrapped_str: &str = "fieldIdWrapped";
// Java field: String headersInLastRefresh
pub(crate) const headersInLastRefresh_str: &str = "headersInLastRefresh";
// Java field: String minimumVersion
pub(crate) const minimumVersion_str: &str = "minimumVersion";
// Java field: String nextId
pub(crate) const nextId_str: &str = "nextId";
// Java field: String unboundColumnsLeft
pub(crate) const unboundColumnsLeft_str: &str = "unboundColumnsLeft";
// Java field: String unboundColumnsRight
pub(crate) const unboundColumnsRight_str: &str = "unboundColumnsRight";
// Java field: String deletedField
pub(crate) const deletedField_str: &str = "deletedField";
// Java field: String queryTableField
pub(crate) const queryTableField_str: &str = "queryTableField";
// Java field: String dataBound
pub(crate) const dataBound_str: &str = "dataBound";
// Java field: String clipped
pub(crate) const clipped_str: &str = "clipped";
// Java field: String tableColumnId
pub(crate) const tableColumnId_str: &str = "tableColumnId";
// Java field: String sst
pub(crate) const sst_str: &str = "sst";
// Java field: String si
pub(crate) const si_str: &str = "si";
// Java field: String uniqueCount
pub(crate) const uniqueCount_str: &str = "uniqueCount";
// Java field: String sb
pub(crate) const sb_str: &str = "sb";
// Java field: String eb
pub(crate) const eb_str: &str = "eb";
// Java field: String rPr
pub(crate) const rPr_str: &str = "rPr";
// Java field: String rFont
pub(crate) const rFont_str: &str = "rFont";
// Java field: String charset
pub(crate) const charset_str: &str = "charset";
// Java field: String family
pub(crate) const family_str: &str = "family";
// Java field: String strike
pub(crate) const strike_str: &str = "strike";
// Java field: String shadow
pub(crate) const shadow_str: &str = "shadow";
// Java field: String condense
pub(crate) const condense_str: &str = "condense";
// Java field: String extend
pub(crate) const extend_str: &str = "extend";
// Java field: String color
pub(crate) const color_str: &str = "color";
// Java field: String sz
pub(crate) const sz_str: &str = "sz";
// Java field: String vertAlign
pub(crate) const vertAlign_str: &str = "vertAlign";
// Java field: String scheme
pub(crate) const scheme_str: &str = "scheme";
// Java field: String rPh
pub(crate) const rPh_str: &str = "rPh";
// Java field: String phoneticPr
pub(crate) const phoneticPr_str: &str = "phoneticPr";
// Java field: String fontId
pub(crate) const fontId_str: &str = "fontId";
// Java field: String alignment
pub(crate) const alignment_str: &str = "alignment";
// Java field: String revisions
pub(crate) const revisions_str: &str = "revisions";
// Java field: String header
pub(crate) const header_str: &str = "header";
// Java field: String lastGuid
pub(crate) const lastGuid_str: &str = "lastGuid";
// Java field: String shared
pub(crate) const shared_str: &str = "shared";
// Java field: String diskRevisions
pub(crate) const diskRevisions_str: &str = "diskRevisions";
// Java field: String history
pub(crate) const history_str: &str = "history";
// Java field: String trackRevisions
pub(crate) const trackRevisions_str: &str = "trackRevisions";
// Java field: String exclusive
pub(crate) const exclusive_str: &str = "exclusive";
// Java field: String revisionId
pub(crate) const revisionId_str: &str = "revisionId";
// Java field: String version
pub(crate) const version_str: &str = "version";
// Java field: String keepChangeHistory
pub(crate) const keepChangeHistory_str: &str = "keepChangeHistory";
// Java field: String protected2
pub(crate) const protected2_str: &str = "protected";
// Java field: String preserveHistory
pub(crate) const preserveHistory_str: &str = "preserveHistory";
// Java field: String rrc
pub(crate) const rrc_str: &str = "rrc";
// Java field: String rm
pub(crate) const rm_str: &str = "rm";
// Java field: String rcv
pub(crate) const rcv_str: &str = "rcv";
// Java field: String rsnm
pub(crate) const rsnm_str: &str = "rsnm";
// Java field: String ris
pub(crate) const ris_str: &str = "ris";
// Java field: String rcc
pub(crate) const rcc_str: &str = "rcc";
// Java field: String rfmt
pub(crate) const rfmt_str: &str = "rfmt";
// Java field: String raf
pub(crate) const raf_str: &str = "raf";
// Java field: String rdn
pub(crate) const rdn_str: &str = "rdn";
// Java field: String rcmt
pub(crate) const rcmt_str: &str = "rcmt";
// Java field: String rqt
pub(crate) const rqt_str: &str = "rqt";
// Java field: String rcft
pub(crate) const rcft_str: &str = "rcft";
// Java field: String rId
pub(crate) const rId_str: &str = "rId";
// Java field: String ua
pub(crate) const ua_str: &str = "ua";
// Java field: String ra
pub(crate) const ra_str: &str = "ra";
// Java field: String sheetIdMap
pub(crate) const sheetIdMap_str: &str = "sheetIdMap";
// Java field: String reviewedList
pub(crate) const reviewedList_str: &str = "reviewedList";
// Java field: String dateTime
pub(crate) const dateTime_str: &str = "dateTime";
// Java field: String maxSheetId
pub(crate) const maxSheetId_str: &str = "maxSheetId";
// Java field: String userName
pub(crate) const userName_str: &str = "userName";
// Java field: String minRId
pub(crate) const minRId_str: &str = "minRId";
// Java field: String maxRId
pub(crate) const maxRId_str: &str = "maxRId";
// Java field: String sheetId
pub(crate) const sheetId_str: &str = "sheetId";
// Java field: String reviewed
pub(crate) const reviewed_str: &str = "reviewed";
// Java field: String index
pub(crate) const index_str: &str = "index";
// Java field: String exp
pub(crate) const exp_str: &str = "exp";
// Java field: String ref3D
pub(crate) const ref3D_str: &str = "ref3D";
// Java field: String array
pub(crate) const array_str: &str = "array";
// Java field: String nf
pub(crate) const nf_str: &str = "nf";
// Java field: String cs
pub(crate) const cs_str: &str = "cs";
// Java field: String dr
pub(crate) const dr_str: &str = "dr";
// Java field: String dn
pub(crate) const dn_str: &str = "dn";
// Java field: String sId
pub(crate) const sId_str: &str = "sId";
// Java field: String undo
pub(crate) const undo_str: &str = "undo";
// Java field: String eol
pub(crate) const eol_str: &str = "eol";
// Java field: String edge
pub(crate) const edge_str: &str = "edge";
// Java field: String source
pub(crate) const source_str: &str = "source";
// Java field: String destination
pub(crate) const destination_str: &str = "destination";
// Java field: String sourceSheetId
pub(crate) const sourceSheetId_str: &str = "sourceSheetId";
// Java field: String oldName
pub(crate) const oldName_str: &str = "oldName";
// Java field: String newName
pub(crate) const newName_str: &str = "newName";
// Java field: String sheetPosition
pub(crate) const sheetPosition_str: &str = "sheetPosition";
// Java field: String oc
pub(crate) const oc_str: &str = "oc";
// Java field: String nc
pub(crate) const nc_str: &str = "nc";
// Java field: String odxf
pub(crate) const odxf_str: &str = "odxf";
// Java field: String ndxf
pub(crate) const ndxf_str: &str = "ndxf";
// Java field: String xfDxf
pub(crate) const xfDxf_str: &str = "xfDxf";
// Java field: String dxf
pub(crate) const dxf_str: &str = "dxf";
// Java field: String quotePrefix
pub(crate) const quotePrefix_str: &str = "quotePrefix";
// Java field: String oldQuotePrefix
pub(crate) const oldQuotePrefix_str: &str = "oldQuotePrefix";
// Java field: String ph
pub(crate) const ph_str: &str = "ph";
// Java field: String oldPh
pub(crate) const oldPh_str: &str = "oldPh";
// Java field: String endOfListFormulaUpdate
pub(crate) const endOfListFormulaUpdate_str: &str = "endOfListFormulaUpdate";
// Java field: String sqref
pub(crate) const sqref_str: &str = "sqref";
// Java field: String start
pub(crate) const start_str: &str = "start";
// Java field: String length
pub(crate) const length_str: &str = "length";
// Java field: String alwaysShow
pub(crate) const alwaysShow_str: &str = "alwaysShow";
// Java field: String old
pub(crate) const old_str: &str = "old";
// Java field: String hiddenRow
pub(crate) const hiddenRow_str: &str = "hiddenRow";
// Java field: String hiddenColumn
pub(crate) const hiddenColumn_str: &str = "hiddenColumn";
// Java field: String oldLength
pub(crate) const oldLength_str: &str = "oldLength";
// Java field: String newLength
pub(crate) const newLength_str: &str = "newLength";
// Java field: String oldFormula
pub(crate) const oldFormula_str: &str = "oldFormula";
// Java field: String localSheetId
pub(crate) const localSheetId_str: &str = "localSheetId";
// Java field: String customView
pub(crate) const customView_str: &str = "customView";
// Java field: String function
pub(crate) const function_str: &str = "function";
// Java field: String oldFunction
pub(crate) const oldFunction_str: &str = "oldFunction";
// Java field: String functionGroupId
pub(crate) const functionGroupId_str: &str = "functionGroupId";
// Java field: String oldFunctionGroupId
pub(crate) const oldFunctionGroupId_str: &str = "oldFunctionGroupId";
// Java field: String shortcutKey
pub(crate) const shortcutKey_str: &str = "shortcutKey";
// Java field: String oldShortcutKey
pub(crate) const oldShortcutKey_str: &str = "oldShortcutKey";
// Java field: String oldHidden
pub(crate) const oldHidden_str: &str = "oldHidden";
// Java field: String customMenu
pub(crate) const customMenu_str: &str = "customMenu";
// Java field: String oldCustomMenu
pub(crate) const oldCustomMenu_str: &str = "oldCustomMenu";
// Java field: String oldDescription
pub(crate) const oldDescription_str: &str = "oldDescription";
// Java field: String help
pub(crate) const help_str: &str = "help";
// Java field: String oldHelp
pub(crate) const oldHelp_str: &str = "oldHelp";
// Java field: String statusBar
pub(crate) const statusBar_str: &str = "statusBar";
// Java field: String oldStatusBar
pub(crate) const oldStatusBar_str: &str = "oldStatusBar";
// Java field: String oldComment
pub(crate) const oldComment_str: &str = "oldComment";
// Java field: String fieldId
pub(crate) const fieldId_str: &str = "fieldId";
// Java field: String users
pub(crate) const users_str: &str = "users";
// Java field: String userInfo
pub(crate) const userInfo_str: &str = "userInfo";
// Java field: String worksheet
pub(crate) const worksheet_str: &str = "worksheet";
// Java field: String chartsheet
pub(crate) const chartsheet_str: &str = "chartsheet";
// Java field: String dialogsheet
pub(crate) const dialogsheet_str: &str = "dialogsheet";
// Java field: String sheetPr
pub(crate) const sheetPr_str: &str = "sheetPr";
// Java field: String sheetViews
pub(crate) const sheetViews_str: &str = "sheetViews";
// Java field: String sheetFormatPr
pub(crate) const sheetFormatPr_str: &str = "sheetFormatPr";
// Java field: String cols
pub(crate) const cols_str: &str = "cols";
// Java field: String sheetData
pub(crate) const sheetData_str: &str = "sheetData";
// Java field: String sheetProtection
pub(crate) const sheetProtection_str: &str = "sheetProtection";
// Java field: String dataConsolidate
pub(crate) const dataConsolidate_str: &str = "dataConsolidate";
// Java field: String customSheetViews
pub(crate) const customSheetViews_str: &str = "customSheetViews";
// Java field: String conditionalFormatting
pub(crate) const conditionalFormatting_str: &str = "conditionalFormatting";
// Java field: String conditionalFormattings
pub(crate) const conditionalFormattings_str: &str = "conditionalFormattings";
// Java field: String printOptions
pub(crate) const printOptions_str: &str = "printOptions";
// Java field: String pageMargins
pub(crate) const pageMargins_str: &str = "pageMargins";
// Java field: String pageSetup
pub(crate) const pageSetup_str: &str = "pageSetup";
// Java field: String headerFooter
pub(crate) const headerFooter_str: &str = "headerFooter";
// Java field: String rowBreaks
pub(crate) const rowBreaks_str: &str = "rowBreaks";
// Java field: String colBreaks
pub(crate) const colBreaks_str: &str = "colBreaks";
// Java field: String customProperties
pub(crate) const customProperties_str: &str = "customProperties";
// Java field: String drawing
pub(crate) const drawing_str: &str = "drawing";
// Java field: String legacyDrawing
pub(crate) const legacyDrawing_str: &str = "legacyDrawing";
// Java field: String legacyDrawingHF
pub(crate) const legacyDrawingHF_str: &str = "legacyDrawingHF";
// Java field: String drawingHF
pub(crate) const drawingHF_str: &str = "drawingHF";
// Java field: String picture
pub(crate) const picture_str: &str = "picture";
// Java field: String oleObjects
pub(crate) const oleObjects_str: &str = "oleObjects";
// Java field: String sheetCalcPr
pub(crate) const sheetCalcPr_str: &str = "sheetCalcPr";
// Java field: String protectedRanges
pub(crate) const protectedRanges_str: &str = "protectedRanges";
// Java field: String scenarios
pub(crate) const scenarios_str: &str = "scenarios";
// Java field: String mergeCells
pub(crate) const mergeCells_str: &str = "mergeCells";
// Java field: String dataValidations
pub(crate) const dataValidations_str: &str = "dataValidations";
// Java field: String hyperlinks
pub(crate) const hyperlinks_str: &str = "hyperlinks";
// Java field: String cellWatches
pub(crate) const cellWatches_str: &str = "cellWatches";
// Java field: String ignoredErrors
pub(crate) const ignoredErrors_str: &str = "ignoredErrors";
// Java field: String smartTags
pub(crate) const smartTags_str: &str = "smartTags";
// Java field: String controls
pub(crate) const controls_str: &str = "controls";
// Java field: String webPublishItems
pub(crate) const webPublishItems_str: &str = "webPublishItems";
// Java field: String tableParts
pub(crate) const tableParts_str: &str = "tableParts";
// Java field: String row
pub(crate) const row_str: &str = "row";
// Java field: String fullCalcOnLoad
pub(crate) const fullCalcOnLoad_str: &str = "fullCalcOnLoad";
// Java field: String baseColWidth
pub(crate) const baseColWidth_str: &str = "baseColWidth";
// Java field: String defaultColWidth
pub(crate) const defaultColWidth_str: &str = "defaultColWidth";
// Java field: String defaultRowHeight
pub(crate) const defaultRowHeight_str: &str = "defaultRowHeight";
// Java field: String customHeight
pub(crate) const customHeight_str: &str = "customHeight";
// Java field: String zeroHeight
pub(crate) const zeroHeight_str: &str = "zeroHeight";
// Java field: String thickTop
pub(crate) const thickTop_str: &str = "thickTop";
// Java field: String thickBottom
pub(crate) const thickBottom_str: &str = "thickBottom";
// Java field: String outlineLevelRow
pub(crate) const outlineLevelRow_str: &str = "outlineLevelRow";
// Java field: String outlineLevelCol
pub(crate) const outlineLevelCol_str: &str = "outlineLevelCol";
// Java field: String col
pub(crate) const col_str: &str = "col";
// Java field: String min
pub(crate) const min_str: &str = "min";
// Java field: String max
pub(crate) const max_str: &str = "max";
// Java field: String width
pub(crate) const width_str: &str = "width";
// Java field: String style
pub(crate) const style_str: &str = "style";
// Java field: String bestFit
pub(crate) const bestFit_str: &str = "bestFit";
// Java field: String customWidth
pub(crate) const customWidth_str: &str = "customWidth";
// Java field: String phonetic
pub(crate) const phonetic_str: &str = "phonetic";
// Java field: String outlineLevel
pub(crate) const outlineLevel_str: &str = "outlineLevel";
// Java field: String collapsed
pub(crate) const collapsed_str: &str = "collapsed";
// Java field: String spans
pub(crate) const spans_str: &str = "spans";
// Java field: String customFormat
pub(crate) const customFormat_str: &str = "customFormat";
// Java field: String ht
pub(crate) const ht_str: &str = "ht";
// Java field: String thickBot
pub(crate) const thickBot_str: &str = "thickBot";
// Java field: String is
pub(crate) const is_str: &str = "is";
// Java field: String cm
pub(crate) const cm_str: &str = "cm";
// Java field: String vm
pub(crate) const vm_str: &str = "vm";
// Java field: String tabColor
pub(crate) const tabColor_str: &str = "tabColor";
// Java field: String outlinePr
pub(crate) const outlinePr_str: &str = "outlinePr";
// Java field: String pageSetUpPr
pub(crate) const pageSetUpPr_str: &str = "pageSetUpPr";
// Java field: String syncHorizontal
pub(crate) const syncHorizontal_str: &str = "syncHorizontal";
// Java field: String syncVertical
pub(crate) const syncVertical_str: &str = "syncVertical";
// Java field: String syncRef
pub(crate) const syncRef_str: &str = "syncRef";
// Java field: String transitionEvaluation
pub(crate) const transitionEvaluation_str: &str = "transitionEvaluation";
// Java field: String transitionEntry
pub(crate) const transitionEntry_str: &str = "transitionEntry";
// Java field: String codeName
pub(crate) const codeName_str: &str = "codeName";
// Java field: String filterMode
pub(crate) const filterMode_str: &str = "filterMode";
// Java field: String enableFormatConditionsCalculation
pub(crate) const enableFormatConditionsCalculation_str: &str = "enableFormatConditionsCalculation";
// Java field: String sheetView
pub(crate) const sheetView_str: &str = "sheetView";
// Java field: String pane
pub(crate) const pane_str: &str = "pane";
// Java field: String selection
pub(crate) const selection_str: &str = "selection";
// Java field: String pivotSelection
pub(crate) const pivotSelection_str: &str = "pivotSelection";
// Java field: String windowProtection
pub(crate) const windowProtection_str: &str = "windowProtection";
// Java field: String showFormulas
pub(crate) const showFormulas_str: &str = "showFormulas";
// Java field: String showGridLines
pub(crate) const showGridLines_str: &str = "showGridLines";
// Java field: String showRowColHeaders
pub(crate) const showRowColHeaders_str: &str = "showRowColHeaders";
// Java field: String showZeros
pub(crate) const showZeros_str: &str = "showZeros";
// Java field: String rightToLeft
pub(crate) const rightToLeft_str: &str = "rightToLeft";
// Java field: String tabSelected
pub(crate) const tabSelected_str: &str = "tabSelected";
// Java field: String showRuler
pub(crate) const showRuler_str: &str = "showRuler";
// Java field: String showOutlineSymbols
pub(crate) const showOutlineSymbols_str: &str = "showOutlineSymbols";
// Java field: String defaultGridColor
pub(crate) const defaultGridColor_str: &str = "defaultGridColor";
// Java field: String showWhiteSpace
pub(crate) const showWhiteSpace_str: &str = "showWhiteSpace";
// Java field: String view
pub(crate) const view_str: &str = "view";
// Java field: String topLeftCell
pub(crate) const topLeftCell_str: &str = "topLeftCell";
// Java field: String colorId
pub(crate) const colorId_str: &str = "colorId";
// Java field: String zoomScale
pub(crate) const zoomScale_str: &str = "zoomScale";
// Java field: String zoomScaleNormal
pub(crate) const zoomScaleNormal_str: &str = "zoomScaleNormal";
// Java field: String zoomScaleSheetLayoutView
pub(crate) const zoomScaleSheetLayoutView_str: &str = "zoomScaleSheetLayoutView";
// Java field: String zoomScalePageLayoutView
pub(crate) const zoomScalePageLayoutView_str: &str = "zoomScalePageLayoutView";
// Java field: String workbookViewId
pub(crate) const workbookViewId_str: &str = "workbookViewId";
// Java field: String xSplit
pub(crate) const xSplit_str: &str = "xSplit";
// Java field: String ySplit
pub(crate) const ySplit_str: &str = "ySplit";
// Java field: String activePane
pub(crate) const activePane_str: &str = "activePane";
// Java field: String state
pub(crate) const state_str: &str = "state";
// Java field: String showHeader
pub(crate) const showHeader_str: &str = "showHeader";
// Java field: String label
pub(crate) const label_str: &str = "label";
// Java field: String data
pub(crate) const data_str: &str = "data";
// Java field: String extendable
pub(crate) const extendable_str: &str = "extendable";
// Java field: String activeRow
pub(crate) const activeRow_str: &str = "activeRow";
// Java field: String activeCol
pub(crate) const activeCol_str: &str = "activeCol";
// Java field: String previousRow
pub(crate) const previousRow_str: &str = "previousRow";
// Java field: String previousCol
pub(crate) const previousCol_str: &str = "previousCol";
// Java field: String click
pub(crate) const click_str: &str = "click";
// Java field: String activeCell
pub(crate) const activeCell_str: &str = "activeCell";
// Java field: String activeCellId
pub(crate) const activeCellId_str: &str = "activeCellId";
// Java field: String brk
pub(crate) const brk_str: &str = "brk";
// Java field: String manualBreakCount
pub(crate) const manualBreakCount_str: &str = "manualBreakCount";
// Java field: String man
pub(crate) const man_str: &str = "man";
// Java field: String pt
pub(crate) const pt_str: &str = "pt";
// Java field: String applyStyles
pub(crate) const applyStyles_str: &str = "applyStyles";
// Java field: String summaryBelow
pub(crate) const summaryBelow_str: &str = "summaryBelow";
// Java field: String summaryRight
pub(crate) const summaryRight_str: &str = "summaryRight";
// Java field: String autoPageBreaks
pub(crate) const autoPageBreaks_str: &str = "autoPageBreaks";
// Java field: String fitToPage
pub(crate) const fitToPage_str: &str = "fitToPage";
// Java field: String dataRefs
pub(crate) const dataRefs_str: &str = "dataRefs";
// Java field: String startLabels
pub(crate) const startLabels_str: &str = "startLabels";
// Java field: String leftLabels
pub(crate) const leftLabels_str: &str = "leftLabels";
// Java field: String topLabels
pub(crate) const topLabels_str: &str = "topLabels";
// Java field: String link
pub(crate) const link_str: &str = "link";
// Java field: String dataRef
pub(crate) const dataRef_str: &str = "dataRef";
// Java field: String mergeCell
pub(crate) const mergeCell_str: &str = "mergeCell";
// Java field: String cellSmartTags
pub(crate) const cellSmartTags_str: &str = "cellSmartTags";
// Java field: String cellSmartTag
pub(crate) const cellSmartTag_str: &str = "cellSmartTag";
// Java field: String cellSmartTagPr
pub(crate) const cellSmartTagPr_str: &str = "cellSmartTagPr";
// Java field: String xmlBased
pub(crate) const xmlBased_str: &str = "xmlBased";
// Java field: String key
pub(crate) const key_str: &str = "key";
// Java field: String lho
pub(crate) const lho_str: &str = "lho";
// Java field: String lhe
pub(crate) const lhe_str: &str = "lhe";
// Java field: String lhf
pub(crate) const lhf_str: &str = "lhf";
// Java field: String cho
pub(crate) const cho_str: &str = "cho";
// Java field: String che
pub(crate) const che_str: &str = "che";
// Java field: String chf
pub(crate) const chf_str: &str = "chf";
// Java field: String rho
pub(crate) const rho_str: &str = "rho";
// Java field: String rhe
pub(crate) const rhe_str: &str = "rhe";
// Java field: String rhf
pub(crate) const rhf_str: &str = "rhf";
// Java field: String lfo
pub(crate) const lfo_str: &str = "lfo";
// Java field: String lfe
pub(crate) const lfe_str: &str = "lfe";
// Java field: String lff
pub(crate) const lff_str: &str = "lff";
// Java field: String cfo
pub(crate) const cfo_str: &str = "cfo";
// Java field: String cfe
pub(crate) const cfe_str: &str = "cfe";
// Java field: String cff
pub(crate) const cff_str: &str = "cff";
// Java field: String rfo
pub(crate) const rfo_str: &str = "rfo";
// Java field: String rfe
pub(crate) const rfe_str: &str = "rfe";
// Java field: String rff
pub(crate) const rff_str: &str = "rff";
// Java field: String customSheetView
pub(crate) const customSheetView_str: &str = "customSheetView";
// Java field: String scale
pub(crate) const scale_str: &str = "scale";
// Java field: String showPageBreaks
pub(crate) const showPageBreaks_str: &str = "showPageBreaks";
// Java field: String showRowCol
pub(crate) const showRowCol_str: &str = "showRowCol";
// Java field: String outlineSymbols
pub(crate) const outlineSymbols_str: &str = "outlineSymbols";
// Java field: String zeroValues
pub(crate) const zeroValues_str: &str = "zeroValues";
// Java field: String printArea
pub(crate) const printArea_str: &str = "printArea";
// Java field: String showAutoFilter
pub(crate) const showAutoFilter_str: &str = "showAutoFilter";
// Java field: String hiddenRows
pub(crate) const hiddenRows_str: &str = "hiddenRows";
// Java field: String hiddenColumns
pub(crate) const hiddenColumns_str: &str = "hiddenColumns";
// Java field: String filterUnique
pub(crate) const filterUnique_str: &str = "filterUnique";
// Java field: String dataValidation
pub(crate) const dataValidation_str: &str = "dataValidation";
// Java field: String disablePrompts
pub(crate) const disablePrompts_str: &str = "disablePrompts";
// Java field: String xWindow
pub(crate) const xWindow_str: &str = "xWindow";
// Java field: String yWindow
pub(crate) const yWindow_str: &str = "yWindow";
// Java field: String formula1
pub(crate) const formula1_str: &str = "formula1";
// Java field: String formula2
pub(crate) const formula2_str: &str = "formula2";
// Java field: String errorStyle
pub(crate) const errorStyle_str: &str = "errorStyle";
// Java field: String imeMode
pub(crate) const imeMode_str: &str = "imeMode";
// Java field: String allowBlank
pub(crate) const allowBlank_str: &str = "allowBlank";
// Java field: String showDropDown
pub(crate) const showDropDown_str: &str = "showDropDown";
// Java field: String showInputMessage
pub(crate) const showInputMessage_str: &str = "showInputMessage";
// Java field: String showErrorMessage
pub(crate) const showErrorMessage_str: &str = "showErrorMessage";
// Java field: String errorTitle
pub(crate) const errorTitle_str: &str = "errorTitle";
// Java field: String error
pub(crate) const error_str: &str = "error";
// Java field: String promptTitle
pub(crate) const promptTitle_str: &str = "promptTitle";
// Java field: String cfRule
pub(crate) const cfRule_str: &str = "cfRule";
// Java field: String pivot
pub(crate) const pivot_str: &str = "pivot";
// Java field: String colorScale
pub(crate) const colorScale_str: &str = "colorScale";
// Java field: String dataBar
pub(crate) const dataBar_str: &str = "dataBar";
// Java field: String stopIfTrue
pub(crate) const stopIfTrue_str: &str = "stopIfTrue";
// Java field: String aboveAverage
pub(crate) const aboveAverage_str: &str = "aboveAverage";
// Java field: String bottom
pub(crate) const bottom_str: &str = "bottom";
// Java field: String timePeriod
pub(crate) const timePeriod_str: &str = "timePeriod";
// Java field: String rank
pub(crate) const rank_str: &str = "rank";
// Java field: String stdDev
pub(crate) const stdDev_str: &str = "stdDev";
// Java field: String equalAverage
pub(crate) const equalAverage_str: &str = "equalAverage";
// Java field: String hyperlink
pub(crate) const hyperlink_str: &str = "hyperlink";
// Java field: String tooltip
pub(crate) const tooltip_str: &str = "tooltip";
// Java field: String display
pub(crate) const display_str: &str = "display";
// Java field: String aca
pub(crate) const aca_str: &str = "aca";
// Java field: String dt2D
pub(crate) const dt2D_str: &str = "dt2D";
// Java field: String dtr
pub(crate) const dtr_str: &str = "dtr";
// Java field: String del1
pub(crate) const del1_str: &str = "del1";
// Java field: String del2
pub(crate) const del2_str: &str = "del2";
// Java field: String r1
pub(crate) const r1_str: &str = "r1";
// Java field: String r2
pub(crate) const r2_str: &str = "r2";
// Java field: String ca
pub(crate) const ca_str: &str = "ca";
// Java field: String bx
pub(crate) const bx_str: &str = "bx";
// Java field: String cfvo
pub(crate) const cfvo_str: &str = "cfvo";
// Java field: String minLength
pub(crate) const minLength_str: &str = "minLength";
// Java field: String maxLength
pub(crate) const maxLength_str: &str = "maxLength";
// Java field: String showValue
pub(crate) const showValue_str: &str = "showValue";
// Java field: String reverse
pub(crate) const reverse_str: &str = "reverse";
// Java field: String gte
pub(crate) const gte_str: &str = "gte";
// Java field: String left
pub(crate) const left_str: &str = "left";
// Java field: String right
pub(crate) const right_str: &str = "right";
// Java field: String footer
pub(crate) const footer_str: &str = "footer";
// Java field: String horizontalCentered
pub(crate) const horizontalCentered_str: &str = "horizontalCentered";
// Java field: String verticalCentered
pub(crate) const verticalCentered_str: &str = "verticalCentered";
// Java field: String headings
pub(crate) const headings_str: &str = "headings";
// Java field: String gridLines
pub(crate) const gridLines_str: &str = "gridLines";
// Java field: String gridLinesSet
pub(crate) const gridLinesSet_str: &str = "gridLinesSet";
// Java field: String paperSize
pub(crate) const paperSize_str: &str = "paperSize";
// Java field: String paperHeight
pub(crate) const paperHeight_str: &str = "paperHeight";
// Java field: String paperWidth
pub(crate) const paperWidth_str: &str = "paperWidth";
// Java field: String firstPageNumber
pub(crate) const firstPageNumber_str: &str = "firstPageNumber";
// Java field: String fitToWidth
pub(crate) const fitToWidth_str: &str = "fitToWidth";
// Java field: String fitToHeight
pub(crate) const fitToHeight_str: &str = "fitToHeight";
// Java field: String pageOrder
pub(crate) const pageOrder_str: &str = "pageOrder";
// Java field: String orientation
pub(crate) const orientation_str: &str = "orientation";
// Java field: String usePrinterDefaults
pub(crate) const usePrinterDefaults_str: &str = "usePrinterDefaults";
// Java field: String blackAndWhite
pub(crate) const blackAndWhite_str: &str = "blackAndWhite";
// Java field: String draft
pub(crate) const draft_str: &str = "draft";
// Java field: String cellComments
pub(crate) const cellComments_str: &str = "cellComments";
// Java field: String useFirstPageNumber
pub(crate) const useFirstPageNumber_str: &str = "useFirstPageNumber";
// Java field: String errors
pub(crate) const errors_str: &str = "errors";
// Java field: String horizontalDpi
pub(crate) const horizontalDpi_str: &str = "horizontalDpi";
// Java field: String verticalDpi
pub(crate) const verticalDpi_str: &str = "verticalDpi";
// Java field: String copies
pub(crate) const copies_str: &str = "copies";
// Java field: String oddHeader
pub(crate) const oddHeader_str: &str = "oddHeader";
// Java field: String oddFooter
pub(crate) const oddFooter_str: &str = "oddFooter";
// Java field: String evenHeader
pub(crate) const evenHeader_str: &str = "evenHeader";
// Java field: String evenFooter
pub(crate) const evenFooter_str: &str = "evenFooter";
// Java field: String firstHeader
pub(crate) const firstHeader_str: &str = "firstHeader";
// Java field: String firstFooter
pub(crate) const firstFooter_str: &str = "firstFooter";
// Java field: String differentOddEven
pub(crate) const differentOddEven_str: &str = "differentOddEven";
// Java field: String differentFirst
pub(crate) const differentFirst_str: &str = "differentFirst";
// Java field: String scaleWithDoc
pub(crate) const scaleWithDoc_str: &str = "scaleWithDoc";
// Java field: String alignWithMargins
pub(crate) const alignWithMargins_str: &str = "alignWithMargins";
// Java field: String scenario
pub(crate) const scenario_str: &str = "scenario";
// Java field: String current
pub(crate) const current_str: &str = "current";
// Java field: String show
pub(crate) const show_str: &str = "show";
// Java field: String password
pub(crate) const password_str: &str = "password";
// Java field: String algorithmName
pub(crate) const algorithmName_str: &str = "algorithmName";
// Java field: String hashValue
pub(crate) const hashValue_str: &str = "hashValue";
// Java field: String saltValue
pub(crate) const saltValue_str: &str = "saltValue";
// Java field: String spinCount
pub(crate) const spinCount_str: &str = "spinCount";
// Java field: String objects
pub(crate) const objects_str: &str = "objects";
// Java field: String formatCells
pub(crate) const formatCells_str: &str = "formatCells";
// Java field: String formatColumns
pub(crate) const formatColumns_str: &str = "formatColumns";
// Java field: String formatRows
pub(crate) const formatRows_str: &str = "formatRows";
// Java field: String insertColumns
pub(crate) const insertColumns_str: &str = "insertColumns";
// Java field: String insertRows
pub(crate) const insertRows_str: &str = "insertRows";
// Java field: String insertHyperlinks
pub(crate) const insertHyperlinks_str: &str = "insertHyperlinks";
// Java field: String deleteColumns
pub(crate) const deleteColumns_str: &str = "deleteColumns";
// Java field: String deleteRows
pub(crate) const deleteRows_str: &str = "deleteRows";
// Java field: String selectLockedCells
pub(crate) const selectLockedCells_str: &str = "selectLockedCells";
// Java field: String sort
pub(crate) const sort_str: &str = "sort";
// Java field: String pivotTables
pub(crate) const pivotTables_str: &str = "pivotTables";
// Java field: String selectUnlockedCells
pub(crate) const selectUnlockedCells_str: &str = "selectUnlockedCells";
// Java field: String protectedRange
pub(crate) const protectedRange_str: &str = "protectedRange";
// Java field: String securityDescriptor
pub(crate) const securityDescriptor_str: &str = "securityDescriptor";
// Java field: String inputCells
pub(crate) const inputCells_str: &str = "inputCells";
// Java field: String undone
pub(crate) const undone_str: &str = "undone";
// Java field: String cellWatch
pub(crate) const cellWatch_str: &str = "cellWatch";
// Java field: String zoomToFit
pub(crate) const zoomToFit_str: &str = "zoomToFit";
// Java field: String content
pub(crate) const content_str: &str = "content";
// Java field: String customPr
pub(crate) const customPr_str: &str = "customPr";
// Java field: String oleObject
pub(crate) const oleObject_str: &str = "oleObject";
// Java field: String objectPr
pub(crate) const objectPr_str: &str = "objectPr";
// Java field: String progId
pub(crate) const progId_str: &str = "progId";
// Java field: String dvAspect
pub(crate) const dvAspect_str: &str = "dvAspect";
// Java field: String oleUpdate
pub(crate) const oleUpdate_str: &str = "oleUpdate";
// Java field: String autoLoad
pub(crate) const autoLoad_str: &str = "autoLoad";
// Java field: String autoPict
pub(crate) const autoPict_str: &str = "autoPict";
// Java field: String macro
pub(crate) const r#macro_str: &str = "macro";
// Java field: String dde
pub(crate) const dde_str: &str = "dde";
// Java field: String webPublishItem
pub(crate) const webPublishItem_str: &str = "webPublishItem";
// Java field: String divId
pub(crate) const divId_str: &str = "divId";
// Java field: String sourceType
pub(crate) const sourceType_str: &str = "sourceType";
// Java field: String sourceRef
pub(crate) const sourceRef_str: &str = "sourceRef";
// Java field: String sourceObject
pub(crate) const sourceObject_str: &str = "sourceObject";
// Java field: String destinationFile
pub(crate) const destinationFile_str: &str = "destinationFile";
// Java field: String title
pub(crate) const title_str: &str = "title";
// Java field: String autoRepublish
pub(crate) const autoRepublish_str: &str = "autoRepublish";
// Java field: String control
pub(crate) const control_str: &str = "control";
// Java field: String controlPr
pub(crate) const controlPr_str: &str = "controlPr";
// Java field: String recalcAlways
pub(crate) const recalcAlways_str: &str = "recalcAlways";
// Java field: String linkedCell
pub(crate) const linkedCell_str: &str = "linkedCell";
// Java field: String listFillRange
pub(crate) const listFillRange_str: &str = "listFillRange";
// Java field: String cf
pub(crate) const cf_str: &str = "cf";
// Java field: String ignoredError
pub(crate) const ignoredError_str: &str = "ignoredError";
// Java field: String evalError
pub(crate) const evalError_str: &str = "evalError";
// Java field: String twoDigitTextYear
pub(crate) const twoDigitTextYear_str: &str = "twoDigitTextYear";
// Java field: String numberStoredAsText
pub(crate) const numberStoredAsText_str: &str = "numberStoredAsText";
// Java field: String formulaRange
pub(crate) const formulaRange_str: &str = "formulaRange";
// Java field: String unlockedFormula
pub(crate) const unlockedFormula_str: &str = "unlockedFormula";
// Java field: String emptyCellReference
pub(crate) const emptyCellReference_str: &str = "emptyCellReference";
// Java field: String listDataValidation
pub(crate) const listDataValidation_str: &str = "listDataValidation";
// Java field: String calculatedColumn
pub(crate) const calculatedColumn_str: &str = "calculatedColumn";
// Java field: String tablePart
pub(crate) const tablePart_str: &str = "tablePart";
// Java field: String metadata
pub(crate) const metadata_str: &str = "metadata";
// Java field: String metadataTypes
pub(crate) const metadataTypes_str: &str = "metadataTypes";
// Java field: String metadataStrings
pub(crate) const metadataStrings_str: &str = "metadataStrings";
// Java field: String mdxMetadata
pub(crate) const mdxMetadata_str: &str = "mdxMetadata";
// Java field: String futureMetadata
pub(crate) const futureMetadata_str: &str = "futureMetadata";
// Java field: String cellMetadata
pub(crate) const cellMetadata_str: &str = "cellMetadata";
// Java field: String valueMetadata
pub(crate) const valueMetadata_str: &str = "valueMetadata";
// Java field: String metadataType
pub(crate) const metadataType_str: &str = "metadataType";
// Java field: String minSupportedVersion
pub(crate) const minSupportedVersion_str: &str = "minSupportedVersion";
// Java field: String ghostRow
pub(crate) const ghostRow_str: &str = "ghostRow";
// Java field: String ghostCol
pub(crate) const ghostCol_str: &str = "ghostCol";
// Java field: String edit
pub(crate) const edit_str: &str = "edit";
// Java field: String delete
pub(crate) const delete_str: &str = "delete";
// Java field: String copy
pub(crate) const copy_str: &str = "copy";
// Java field: String pasteAll
pub(crate) const pasteAll_str: &str = "pasteAll";
// Java field: String pasteFormulas
pub(crate) const pasteFormulas_str: &str = "pasteFormulas";
// Java field: String pasteValues
pub(crate) const pasteValues_str: &str = "pasteValues";
// Java field: String pasteFormats
pub(crate) const pasteFormats_str: &str = "pasteFormats";
// Java field: String pasteComments
pub(crate) const pasteComments_str: &str = "pasteComments";
// Java field: String pasteDataValidation
pub(crate) const pasteDataValidation_str: &str = "pasteDataValidation";
// Java field: String pasteBorders
pub(crate) const pasteBorders_str: &str = "pasteBorders";
// Java field: String pasteColWidths
pub(crate) const pasteColWidths_str: &str = "pasteColWidths";
// Java field: String pasteNumberFormats
pub(crate) const pasteNumberFormats_str: &str = "pasteNumberFormats";
// Java field: String merge
pub(crate) const merge_str: &str = "merge";
// Java field: String splitFirst
pub(crate) const splitFirst_str: &str = "splitFirst";
// Java field: String splitAll
pub(crate) const splitAll_str: &str = "splitAll";
// Java field: String rowColShift
pub(crate) const rowColShift_str: &str = "rowColShift";
// Java field: String clearAll
pub(crate) const clearAll_str: &str = "clearAll";
// Java field: String clearFormats
pub(crate) const clearFormats_str: &str = "clearFormats";
// Java field: String clearContents
pub(crate) const clearContents_str: &str = "clearContents";
// Java field: String clearComments
pub(crate) const clearComments_str: &str = "clearComments";
// Java field: String assign
pub(crate) const assign_str: &str = "assign";
// Java field: String coerce
pub(crate) const coerce_str: &str = "coerce";
// Java field: String adjust
pub(crate) const adjust_str: &str = "adjust";
// Java field: String cellMeta
pub(crate) const cellMeta_str: &str = "cellMeta";
// Java field: String bk
pub(crate) const bk_str: &str = "bk";
// Java field: String rc
pub(crate) const rc_str: &str = "rc";
// Java field: String ms
pub(crate) const ms_str: &str = "ms";
// Java field: String p
pub(crate) const p_str: &str = "p";
// Java field: String k
pub(crate) const k_str: &str = "k";
// Java field: String ct
pub(crate) const ct_str: &str = "ct";
// Java field: String fi
pub(crate) const fi_str: &str = "fi";
// Java field: String ns
pub(crate) const ns_str: &str = "ns";
// Java field: String o
pub(crate) const o_str: &str = "o";
// Java field: String np
pub(crate) const np_str: &str = "np";
// Java field: String singleXmlCells
pub(crate) const singleXmlCells_str: &str = "singleXmlCells";
// Java field: String singleXmlCell
pub(crate) const singleXmlCell_str: &str = "singleXmlCell";
// Java field: String xmlCellPr
pub(crate) const xmlCellPr_str: &str = "xmlCellPr";
// Java field: String xmlPr
pub(crate) const xmlPr_str: &str = "xmlPr";
// Java field: String mapId
pub(crate) const mapId_str: &str = "mapId";
// Java field: String xpath
pub(crate) const xpath_str: &str = "xpath";
// Java field: String xmlDataType
pub(crate) const xmlDataType_str: &str = "xmlDataType";
// Java field: String styleSheet
pub(crate) const styleSheet_str: &str = "styleSheet";
// Java field: String numFmts
pub(crate) const numFmts_str: &str = "numFmts";
// Java field: String fonts
pub(crate) const fonts_str: &str = "fonts";
// Java field: String fills
pub(crate) const fills_str: &str = "fills";
// Java field: String borders
pub(crate) const borders_str: &str = "borders";
// Java field: String cellStyleXfs
pub(crate) const cellStyleXfs_str: &str = "cellStyleXfs";
// Java field: String cellXfs
pub(crate) const cellXfs_str: &str = "cellXfs";
// Java field: String cellStyles
pub(crate) const cellStyles_str: &str = "cellStyles";
// Java field: String dxfs
pub(crate) const dxfs_str: &str = "dxfs";
// Java field: String tableStyles
pub(crate) const tableStyles_str: &str = "tableStyles";
// Java field: String colors
pub(crate) const colors_str: &str = "colors";
// Java field: String horizontal
pub(crate) const horizontal_str: &str = "horizontal";
// Java field: String vertical
pub(crate) const vertical_str: &str = "vertical";
// Java field: String textRotation
pub(crate) const textRotation_str: &str = "textRotation";
// Java field: String wrapText
pub(crate) const wrapText_str: &str = "wrapText";
// Java field: String relativeIndent
pub(crate) const relativeIndent_str: &str = "relativeIndent";
// Java field: String justifyLastLine
pub(crate) const justifyLastLine_str: &str = "justifyLastLine";
// Java field: String shrinkToFit
pub(crate) const shrinkToFit_str: &str = "shrinkToFit";
// Java field: String readingOrder
pub(crate) const readingOrder_str: &str = "readingOrder";
// Java field: String border
pub(crate) const border_str: &str = "border";
// Java field: String end
pub(crate) const end_str: &str = "end";
// Java field: String diagonal
pub(crate) const diagonal_str: &str = "diagonal";
// Java field: String diagonalUp
pub(crate) const diagonalUp_str: &str = "diagonalUp";
// Java field: String diagonalDown
pub(crate) const diagonalDown_str: &str = "diagonalDown";
// Java field: String font
pub(crate) const font_str: &str = "font";
// Java field: String fill
pub(crate) const fill_str: &str = "fill";
// Java field: String patternFill
pub(crate) const patternFill_str: &str = "patternFill";
// Java field: String gradientFill
pub(crate) const gradientFill_str: &str = "gradientFill";
// Java field: String fgColor
pub(crate) const fgColor_str: &str = "fgColor";
// Java field: String bgColor
pub(crate) const bgColor_str: &str = "bgColor";
// Java field: String patternType
pub(crate) const patternType_str: &str = "patternType";
// Java field: String auto
pub(crate) const auto_str: &str = "auto";
// Java field: String indexed
pub(crate) const indexed_str: &str = "indexed";
// Java field: String rgb
pub(crate) const rgb_str: &str = "rgb";
// Java field: String theme
pub(crate) const theme_str: &str = "theme";
// Java field: String tint
pub(crate) const tint_str: &str = "tint";
// Java field: String stop
pub(crate) const stop_str: &str = "stop";
// Java field: String degree
pub(crate) const degree_str: &str = "degree";
// Java field: String numFmt
pub(crate) const numFmt_str: &str = "numFmt";
// Java field: String formatCode
pub(crate) const formatCode_str: &str = "formatCode";
// Java field: String xf
pub(crate) const xf_str: &str = "xf";
// Java field: String protection
pub(crate) const protection_str: &str = "protection";
// Java field: String fillId
pub(crate) const fillId_str: &str = "fillId";
// Java field: String borderId
pub(crate) const borderId_str: &str = "borderId";
// Java field: String xfId
pub(crate) const xfId_str: &str = "xfId";
// Java field: String pivotButton
pub(crate) const pivotButton_str: &str = "pivotButton";
// Java field: String applyNumberFormat
pub(crate) const applyNumberFormat_str: &str = "applyNumberFormat";
// Java field: String applyFont
pub(crate) const applyFont_str: &str = "applyFont";
// Java field: String applyFill
pub(crate) const applyFill_str: &str = "applyFill";
// Java field: String applyBorder
pub(crate) const applyBorder_str: &str = "applyBorder";
// Java field: String applyAlignment
pub(crate) const applyAlignment_str: &str = "applyAlignment";
// Java field: String applyProtection
pub(crate) const applyProtection_str: &str = "applyProtection";
// Java field: String cellStyle
pub(crate) const cellStyle_str: &str = "cellStyle";
// Java field: String builtinId
pub(crate) const builtinId_str: &str = "builtinId";
// Java field: String iLevel
pub(crate) const iLevel_str: &str = "iLevel";
// Java field: String customBuiltin
pub(crate) const customBuiltin_str: &str = "customBuiltin";
// Java field: String indexedColors
pub(crate) const indexedColors_str: &str = "indexedColors";
// Java field: String mruColors
pub(crate) const mruColors_str: &str = "mruColors";
// Java field: String rgbColor
pub(crate) const rgbColor_str: &str = "rgbColor";
// Java field: String tableStyle
pub(crate) const tableStyle_str: &str = "tableStyle";
// Java field: String defaultTableStyle
pub(crate) const defaultTableStyle_str: &str = "defaultTableStyle";
// Java field: String defaultPivotStyle
pub(crate) const defaultPivotStyle_str: &str = "defaultPivotStyle";
// Java field: String tableStyleElement
pub(crate) const tableStyleElement_str: &str = "tableStyleElement";
// Java field: String table
pub(crate) const table_str: &str = "table";
// Java field: String size
pub(crate) const size_str: &str = "size";
// Java field: String autoFormatId
pub(crate) const autoFormatId_str: &str = "autoFormatId";
// Java field: String applyNumberFormats
pub(crate) const applyNumberFormats_str: &str = "applyNumberFormats";
// Java field: String applyBorderFormats
pub(crate) const applyBorderFormats_str: &str = "applyBorderFormats";
// Java field: String applyFontFormats
pub(crate) const applyFontFormats_str: &str = "applyFontFormats";
// Java field: String applyPatternFormats
pub(crate) const applyPatternFormats_str: &str = "applyPatternFormats";
// Java field: String applyAlignmentFormats
pub(crate) const applyAlignmentFormats_str: &str = "applyAlignmentFormats";
// Java field: String applyWidthHeightFormats
pub(crate) const applyWidthHeightFormats_str: &str = "applyWidthHeightFormats";
// Java field: String externalLink
pub(crate) const externalLink_str: &str = "externalLink";
// Java field: String externalBook
pub(crate) const externalBook_str: &str = "externalBook";
// Java field: String ddeLink
pub(crate) const ddeLink_str: &str = "ddeLink";
// Java field: String oleLink
pub(crate) const oleLink_str: &str = "oleLink";
// Java field: String sheetNames
pub(crate) const sheetNames_str: &str = "sheetNames";
// Java field: String definedNames
pub(crate) const definedNames_str: &str = "definedNames";
// Java field: String sheetDataSet
pub(crate) const sheetDataSet_str: &str = "sheetDataSet";
// Java field: String sheetName
pub(crate) const sheetName_str: &str = "sheetName";
// Java field: String definedName
pub(crate) const definedName_str: &str = "definedName";
// Java field: String refersTo
pub(crate) const refersTo_str: &str = "refersTo";
// Java field: String refreshError
pub(crate) const refreshError_str: &str = "refreshError";
// Java field: String ddeItems
pub(crate) const ddeItems_str: &str = "ddeItems";
// Java field: String ddeService
pub(crate) const ddeService_str: &str = "ddeService";
// Java field: String ddeTopic
pub(crate) const ddeTopic_str: &str = "ddeTopic";
// Java field: String ddeItem
pub(crate) const ddeItem_str: &str = "ddeItem";
// Java field: String values
pub(crate) const values_str: &str = "values";
// Java field: String ole
pub(crate) const ole_str: &str = "ole";
// Java field: String advise
pub(crate) const advise_str: &str = "advise";
// Java field: String preferPic
pub(crate) const preferPic_str: &str = "preferPic";
// Java field: String rows
pub(crate) const rows_str: &str = "rows";
// Java field: String oleItems
pub(crate) const oleItems_str: &str = "oleItems";
// Java field: String oleItem
pub(crate) const oleItem_str: &str = "oleItem";
// Java field: String icon
pub(crate) const icon_str: &str = "icon";
// Java field: String tableColumns
pub(crate) const tableColumns_str: &str = "tableColumns";
// Java field: String tableStyleInfo
pub(crate) const tableStyleInfo_str: &str = "tableStyleInfo";
// Java field: String displayName
pub(crate) const displayName_str: &str = "displayName";
// Java field: String tableType
pub(crate) const tableType_str: &str = "tableType";
// Java field: String headerRowCount
pub(crate) const headerRowCount_str: &str = "headerRowCount";
// Java field: String insertRow
pub(crate) const insertRow_str: &str = "insertRow";
// Java field: String insertRowShift
pub(crate) const insertRowShift_str: &str = "insertRowShift";
// Java field: String totalsRowCount
pub(crate) const totalsRowCount_str: &str = "totalsRowCount";
// Java field: String totalsRowShown
pub(crate) const totalsRowShown_str: &str = "totalsRowShown";
// Java field: String headerRowDxfId
pub(crate) const headerRowDxfId_str: &str = "headerRowDxfId";
// Java field: String dataDxfId
pub(crate) const dataDxfId_str: &str = "dataDxfId";
// Java field: String totalsRowDxfId
pub(crate) const totalsRowDxfId_str: &str = "totalsRowDxfId";
// Java field: String headerRowBorderDxfId
pub(crate) const headerRowBorderDxfId_str: &str = "headerRowBorderDxfId";
// Java field: String tableBorderDxfId
pub(crate) const tableBorderDxfId_str: &str = "tableBorderDxfId";
// Java field: String totalsRowBorderDxfId
pub(crate) const totalsRowBorderDxfId_str: &str = "totalsRowBorderDxfId";
// Java field: String headerRowCellStyle
pub(crate) const headerRowCellStyle_str: &str = "headerRowCellStyle";
// Java field: String dataCellStyle
pub(crate) const dataCellStyle_str: &str = "dataCellStyle";
// Java field: String totalsRowCellStyle
pub(crate) const totalsRowCellStyle_str: &str = "totalsRowCellStyle";
// Java field: String showFirstColumn
pub(crate) const showFirstColumn_str: &str = "showFirstColumn";
// Java field: String showColumnStripes
pub(crate) const showColumnStripes_str: &str = "showColumnStripes";
// Java field: String tableColumn
pub(crate) const tableColumn_str: &str = "tableColumn";
// Java field: String calculatedColumnFormula
pub(crate) const calculatedColumnFormula_str: &str = "calculatedColumnFormula";
// Java field: String totalsRowFormula
pub(crate) const totalsRowFormula_str: &str = "totalsRowFormula";
// Java field: String xmlColumnPr
pub(crate) const xmlColumnPr_str: &str = "xmlColumnPr";
// Java field: String totalsRowFunction
pub(crate) const totalsRowFunction_str: &str = "totalsRowFunction";
// Java field: String totalsRowLabel
pub(crate) const totalsRowLabel_str: &str = "totalsRowLabel";
// Java field: String queryTableFieldId
pub(crate) const queryTableFieldId_str: &str = "queryTableFieldId";
// Java field: String denormalized
pub(crate) const denormalized_str: &str = "denormalized";
// Java field: String volTypes
pub(crate) const volTypes_str: &str = "volTypes";
// Java field: String volType
pub(crate) const volType_str: &str = "volType";
// Java field: String main
pub(crate) const main_str: &str = "main";
// Java field: String tp
pub(crate) const tp_str: &str = "tp";
// Java field: String first
pub(crate) const first_str: &str = "first";
// Java field: String stp
pub(crate) const stp_str: &str = "stp";
// Java field: String tr
pub(crate) const tr_str: &str = "tr";
// Java field: String workbook
pub(crate) const workbook_str: &str = "workbook";
// Java field: String fileVersion
pub(crate) const fileVersion_str: &str = "fileVersion";
// Java field: String fileSharing
pub(crate) const fileSharing_str: &str = "fileSharing";
// Java field: String workbookPr
pub(crate) const workbookPr_str: &str = "workbookPr";
// Java field: String workbookProtection
pub(crate) const workbookProtection_str: &str = "workbookProtection";
// Java field: String bookViews
pub(crate) const bookViews_str: &str = "bookViews";
// Java field: String sheets
pub(crate) const sheets_str: &str = "sheets";
// Java field: String functionGroups
pub(crate) const functionGroups_str: &str = "functionGroups";
// Java field: String externalReferences
pub(crate) const externalReferences_str: &str = "externalReferences";
// Java field: String calcPr
pub(crate) const calcPr_str: &str = "calcPr";
// Java field: String oleSize
pub(crate) const oleSize_str: &str = "oleSize";
// Java field: String customWorkbookViews
pub(crate) const customWorkbookViews_str: &str = "customWorkbookViews";
// Java field: String pivotCaches
pub(crate) const pivotCaches_str: &str = "pivotCaches";
// Java field: String smartTagPr
pub(crate) const smartTagPr_str: &str = "smartTagPr";
// Java field: String smartTagTypes
pub(crate) const smartTagTypes_str: &str = "smartTagTypes";
// Java field: String webPublishing
pub(crate) const webPublishing_str: &str = "webPublishing";
// Java field: String fileRecoveryPr
pub(crate) const fileRecoveryPr_str: &str = "fileRecoveryPr";
// Java field: String webPublishObjects
pub(crate) const webPublishObjects_str: &str = "webPublishObjects";
// Java field: String conformance
pub(crate) const conformance_str: &str = "conformance";
// Java field: String appName
pub(crate) const appName_str: &str = "appName";
// Java field: String lastEdited
pub(crate) const lastEdited_str: &str = "lastEdited";
// Java field: String lowestEdited
pub(crate) const lowestEdited_str: &str = "lowestEdited";
// Java field: String rupBuild
pub(crate) const rupBuild_str: &str = "rupBuild";
// Java field: String workbookView
pub(crate) const workbookView_str: &str = "workbookView";
// Java field: String visibility
pub(crate) const visibility_str: &str = "visibility";
// Java field: String minimized
pub(crate) const minimized_str: &str = "minimized";
// Java field: String showHorizontalScroll
pub(crate) const showHorizontalScroll_str: &str = "showHorizontalScroll";
// Java field: String showVerticalScroll
pub(crate) const showVerticalScroll_str: &str = "showVerticalScroll";
// Java field: String showSheetTabs
pub(crate) const showSheetTabs_str: &str = "showSheetTabs";
// Java field: String windowWidth
pub(crate) const windowWidth_str: &str = "windowWidth";
// Java field: String windowHeight
pub(crate) const windowHeight_str: &str = "windowHeight";
// Java field: String tabRatio
pub(crate) const tabRatio_str: &str = "tabRatio";
// Java field: String firstSheet
pub(crate) const firstSheet_str: &str = "firstSheet";
// Java field: String activeTab
pub(crate) const activeTab_str: &str = "activeTab";
// Java field: String autoFilterDateGrouping
pub(crate) const autoFilterDateGrouping_str: &str = "autoFilterDateGrouping";
// Java field: String customWorkbookView
pub(crate) const customWorkbookView_str: &str = "customWorkbookView";
// Java field: String autoUpdate
pub(crate) const autoUpdate_str: &str = "autoUpdate";
// Java field: String mergeInterval
pub(crate) const mergeInterval_str: &str = "mergeInterval";
// Java field: String changesSavedWin
pub(crate) const changesSavedWin_str: &str = "changesSavedWin";
// Java field: String onlySync
pub(crate) const onlySync_str: &str = "onlySync";
// Java field: String personalView
pub(crate) const personalView_str: &str = "personalView";
// Java field: String includePrintSettings
pub(crate) const includePrintSettings_str: &str = "includePrintSettings";
// Java field: String includeHiddenRowCol
pub(crate) const includeHiddenRowCol_str: &str = "includeHiddenRowCol";
// Java field: String maximized
pub(crate) const maximized_str: &str = "maximized";
// Java field: String activeSheetId
pub(crate) const activeSheetId_str: &str = "activeSheetId";
// Java field: String showFormulaBar
pub(crate) const showFormulaBar_str: &str = "showFormulaBar";
// Java field: String showStatusbar
pub(crate) const showStatusbar_str: &str = "showStatusbar";
// Java field: String showComments
pub(crate) const showComments_str: &str = "showComments";
// Java field: String showObjects
pub(crate) const showObjects_str: &str = "showObjects";
// Java field: String date1904
pub(crate) const date1904_str: &str = "date1904";
// Java field: String dateCompatibility
pub(crate) const dateCompatibility_str: &str = "dateCompatibility";
// Java field: String showBorderUnselectedTables
pub(crate) const showBorderUnselectedTables_str: &str = "showBorderUnselectedTables";
// Java field: String filterPrivacy
pub(crate) const filterPrivacy_str: &str = "filterPrivacy";
// Java field: String promptedSolutions
pub(crate) const promptedSolutions_str: &str = "promptedSolutions";
// Java field: String showInkAnnotation
pub(crate) const showInkAnnotation_str: &str = "showInkAnnotation";
// Java field: String backupFile
pub(crate) const backupFile_str: &str = "backupFile";
// Java field: String saveExternalLinkValues
pub(crate) const saveExternalLinkValues_str: &str = "saveExternalLinkValues";
// Java field: String updateLinks
pub(crate) const updateLinks_str: &str = "updateLinks";
// Java field: String hidePivotFieldList
pub(crate) const hidePivotFieldList_str: &str = "hidePivotFieldList";
// Java field: String showPivotChartFilter
pub(crate) const showPivotChartFilter_str: &str = "showPivotChartFilter";
// Java field: String allowRefreshQuery
pub(crate) const allowRefreshQuery_str: &str = "allowRefreshQuery";
// Java field: String publishItems
pub(crate) const publishItems_str: &str = "publishItems";
// Java field: String checkCompatibility
pub(crate) const checkCompatibility_str: &str = "checkCompatibility";
// Java field: String autoCompressPictures
pub(crate) const autoCompressPictures_str: &str = "autoCompressPictures";
// Java field: String refreshAllConnections
pub(crate) const refreshAllConnections_str: &str = "refreshAllConnections";
// Java field: String defaultThemeVersion
pub(crate) const defaultThemeVersion_str: &str = "defaultThemeVersion";
// Java field: String embed
pub(crate) const embed_str: &str = "embed";
// Java field: String smartTagType
pub(crate) const smartTagType_str: &str = "smartTagType";
// Java field: String namespaceUri
pub(crate) const namespaceUri_str: &str = "namespaceUri";
// Java field: String autoRecover
pub(crate) const autoRecover_str: &str = "autoRecover";
// Java field: String crashSave
pub(crate) const crashSave_str: &str = "crashSave";
// Java field: String dataExtractLoad
pub(crate) const dataExtractLoad_str: &str = "dataExtractLoad";
// Java field: String repairLoad
pub(crate) const repairLoad_str: &str = "repairLoad";
// Java field: String calcId
pub(crate) const calcId_str: &str = "calcId";
// Java field: String calcMode
pub(crate) const calcMode_str: &str = "calcMode";
// Java field: String refMode
pub(crate) const refMode_str: &str = "refMode";
// Java field: String iterate
pub(crate) const iterate_str: &str = "iterate";
// Java field: String iterateCount
pub(crate) const iterateCount_str: &str = "iterateCount";
// Java field: String iterateDelta
pub(crate) const iterateDelta_str: &str = "iterateDelta";
// Java field: String fullPrecision
pub(crate) const fullPrecision_str: &str = "fullPrecision";
// Java field: String calcCompleted
pub(crate) const calcCompleted_str: &str = "calcCompleted";
// Java field: String calcOnSave
pub(crate) const calcOnSave_str: &str = "calcOnSave";
// Java field: String concurrentCalc
pub(crate) const concurrentCalc_str: &str = "concurrentCalc";
// Java field: String concurrentManualCount
pub(crate) const concurrentManualCount_str: &str = "concurrentManualCount";
// Java field: String forceFullCalc
pub(crate) const forceFullCalc_str: &str = "forceFullCalc";
// Java field: String vbProcedure
pub(crate) const vbProcedure_str: &str = "vbProcedure";
// Java field: String xlm
pub(crate) const xlm_str: &str = "xlm";
// Java field: String publishToServer
pub(crate) const publishToServer_str: &str = "publishToServer";
// Java field: String workbookParameter
pub(crate) const workbookParameter_str: &str = "workbookParameter";
// Java field: String externalReference
pub(crate) const externalReference_str: &str = "externalReference";
// Java field: String pivotCache
pub(crate) const pivotCache_str: &str = "pivotCache";
// Java field: String readOnlyRecommended
pub(crate) const readOnlyRecommended_str: &str = "readOnlyRecommended";
// Java field: String reservationPassword
pub(crate) const reservationPassword_str: &str = "reservationPassword";
// Java field: String workbookPassword
pub(crate) const workbookPassword_str: &str = "workbookPassword";
// Java field: String workbookPasswordCharacterSet
pub(crate) const workbookPasswordCharacterSet_str: &str = "workbookPasswordCharacterSet";
// Java field: String revisionsPassword
pub(crate) const revisionsPassword_str: &str = "revisionsPassword";
// Java field: String revisionsPasswordCharacterSet
pub(crate) const revisionsPasswordCharacterSet_str: &str = "revisionsPasswordCharacterSet";
// Java field: String lockStructure
pub(crate) const lockStructure_str: &str = "lockStructure";
// Java field: String lockWindows
pub(crate) const lockWindows_str: &str = "lockWindows";
// Java field: String lockRevision
pub(crate) const lockRevision_str: &str = "lockRevision";
// Java field: String revisionsAlgorithmName
pub(crate) const revisionsAlgorithmName_str: &str = "revisionsAlgorithmName";
// Java field: String revisionsHashValue
pub(crate) const revisionsHashValue_str: &str = "revisionsHashValue";
// Java field: String revisionsSaltValue
pub(crate) const revisionsSaltValue_str: &str = "revisionsSaltValue";
// Java field: String revisionsSpinCount
pub(crate) const revisionsSpinCount_str: &str = "revisionsSpinCount";
// Java field: String workbookAlgorithmName
pub(crate) const workbookAlgorithmName_str: &str = "workbookAlgorithmName";
// Java field: String workbookHashValue
pub(crate) const workbookHashValue_str: &str = "workbookHashValue";
// Java field: String workbookSaltValue
pub(crate) const workbookSaltValue_str: &str = "workbookSaltValue";
// Java field: String workbookSpinCount
pub(crate) const workbookSpinCount_str: &str = "workbookSpinCount";
// Java field: String css
pub(crate) const css_str: &str = "css";
// Java field: String thicket
pub(crate) const thicket_str: &str = "thicket";
// Java field: String longFileNames
pub(crate) const longFileNames_str: &str = "longFileNames";
// Java field: String vml
pub(crate) const vml_str: &str = "vml";
// Java field: String allowPng
pub(crate) const allowPng_str: &str = "allowPng";
// Java field: String targetScreenSize
pub(crate) const targetScreenSize_str: &str = "targetScreenSize";
// Java field: String dpi
pub(crate) const dpi_str: &str = "dpi";
// Java field: String functionGroup
pub(crate) const functionGroup_str: &str = "functionGroup";
// Java field: String builtInGroupCount
pub(crate) const builtInGroupCount_str: &str = "builtInGroupCount";
// Java field: String webPublishObject
pub(crate) const webPublishObject_str: &str = "webPublishObject";
// Java field: String sparklineGroup
pub(crate) const sparklineGroup_str: &str = "sparklineGroup";
// Java field: String sparklineGroups
pub(crate) const sparklineGroups_str: &str = "sparklineGroups";
// Java field: String colorSeries
pub(crate) const colorSeries_str: &str = "colorSeries";
// Java field: String colorNegative
pub(crate) const colorNegative_str: &str = "colorNegative";
// Java field: String colorAxis
pub(crate) const colorAxis_str: &str = "colorAxis";
// Java field: String colorMarkers
pub(crate) const colorMarkers_str: &str = "colorMarkers";
// Java field: String colorFirst
pub(crate) const colorFirst_str: &str = "colorFirst";
// Java field: String colorLast
pub(crate) const colorLast_str: &str = "colorLast";
// Java field: String colorHigh
pub(crate) const colorHigh_str: &str = "colorHigh";
// Java field: String colorLow
pub(crate) const colorLow_str: &str = "colorLow";
// Java field: String sparkline
pub(crate) const sparkline_str: &str = "sparkline";
// Java field: String sparklines
pub(crate) const sparklines_str: &str = "sparklines";
// Java field: String minAxisType
pub(crate) const minAxisType_str: &str = "minAxisType";
// Java field: String manualMin
pub(crate) const manualMin_str: &str = "manualMin";
// Java field: String maxAxisType
pub(crate) const maxAxisType_str: &str = "maxAxisType";
// Java field: String manualMax
pub(crate) const manualMax_str: &str = "manualMax";
// Java field: String displayXAxis
pub(crate) const displayXAxis_str: &str = "displayXAxis";
// Java field: String displayEmptyCellsAs
pub(crate) const displayEmptyCellsAs_str: &str = "displayEmptyCellsAs";
// Java field: String last
pub(crate) const last_str: &str = "last";
// Java field: String high
pub(crate) const high_str: &str = "high";
// Java field: String low
pub(crate) const low_str: &str = "low";
// Java field: String negative
pub(crate) const negative_str: &str = "negative";
// Java field: String markers
pub(crate) const markers_str: &str = "markers";
// Java field: String displayHidden
pub(crate) const displayHidden_str: &str = "displayHidden";
// Java field: String lineWeight
pub(crate) const lineWeight_str: &str = "lineWeight";
// Java field: String dateAxis
pub(crate) const dateAxis_str: &str = "dateAxis";
// Java field: String axisColor
pub(crate) const axisColor_str: &str = "axisColor";
// Java field: String allowEditUser
pub(crate) const allowEditUser_str: &str = "allowEditUser";
// Java field: String hasInvisiblePropRange
pub(crate) const hasInvisiblePropRange_str: &str = "hasInvisiblePropRange";
// Java field: String rangeList
pub(crate) const rangeList_str: &str = "rangeList";
// Java field: String sheetStid
pub(crate) const sheetStid_str: &str = "sheetStid";
// Java field: String master
pub(crate) const master_str: &str = "master";
// Java field: String pwdUUID
pub(crate) const pwdUUID_str: &str = "pwdUUID";
// Java field: String arrUserId
pub(crate) const arrUserId_str: &str = "arrUserId";
// Java field: String rangeCreator
pub(crate) const rangeCreator_str: &str = "rangeCreator";
// Java field: String othersAccessPermission
pub(crate) const othersAccessPermission_str: &str = "othersAccessPermission";
// Java field: String userID
pub(crate) const userID_str: &str = "userID";
// Java field: String accessPermission
pub(crate) const accessPermission_str: &str = "accessPermission";
// Java field: String woProps
pub(crate) const woProps_str: &str = "woProps";
// Java field: String woSheetsProps
pub(crate) const woSheetsProps_str: &str = "woSheetsProps";
// Java field: String woSheetProps
pub(crate) const woSheetProps_str: &str = "woSheetProps";
// Java field: String interlineOnOff
pub(crate) const interlineOnOff_str: &str = "interlineOnOff";
// Java field: String interlineColor
pub(crate) const interlineColor_str: &str = "interlineColor";
// Java field: String isDbSheet
pub(crate) const isDbSheet_str: &str = "isDbSheet";
// Java field: String woBookProps
pub(crate) const woBookProps_str: &str = "woBookProps";
// Java field: String bookSettings
pub(crate) const bookSettings_str: &str = "bookSettings";
// Java field: String isFilterShared
pub(crate) const isFilterShared_str: &str = "isFilterShared";
// Java field: String isAutoUpdatePaused
pub(crate) const isAutoUpdatePaused_str: &str = "isAutoUpdatePaused";
// Java field: String filterType
pub(crate) const filterType_str: &str = "filterType";
// Java field: String isMergeTasksAutoUpdate
pub(crate) const isMergeTasksAutoUpdate_str: &str = "isMergeTasksAutoUpdate";
// Java field: String dashboard
pub(crate) const dashboard_str: &str = "dashboard";

// Java: public class SMap implements XmlName2Id
pub struct SMap {
    // Java field: SMap obj
    // Java field: StringIntMap mapobj
    pub(crate) mapobj: HashMap<i32, i32>,
}
static OBJ: OnceLock<SMap> = OnceLock::new();

// Java: public class SMap implements XmlName2Id
impl SMap {
    // Java: void init()
    fn init(&mut self) {
      // Java: this.mapobj.put("Properties", 5397);
      self.mapobj.insert(hash_str("Properties"), 5397);
      // Java: this.mapobj.put("property", 5398);
      self.mapobj.insert(hash_str("property"), 5398);
      // Java: this.mapobj.put("bool", S.bool);
      self.mapobj.insert(hash_str("bool"), 5399);
      // Java: this.mapobj.put("filterColumn", 4097);
      self.mapobj.insert(hash_str("filterColumn"), 4097);
      // Java: this.mapobj.put("sortState", 4098);
      self.mapobj.insert(hash_str("sortState"), 4098);
      // Java: this.mapobj.put("extLst", 4099);
      self.mapobj.insert(hash_str("extLst"), 4099);
      // Java: this.mapobj.put("ref", 4100);
      self.mapobj.insert(hash_str("ref"), 4100);
      // Java: this.mapobj.put("filters", 4101);
      self.mapobj.insert(hash_str("filters"), 4101);
      // Java: this.mapobj.put("top10", 4102);
      self.mapobj.insert(hash_str("top10"), 4102);
      // Java: this.mapobj.put("customFilters", 4103);
      self.mapobj.insert(hash_str("customFilters"), 4103);
      // Java: this.mapobj.put("dynamicFilter", 4104);
      self.mapobj.insert(hash_str("dynamicFilter"), 4104);
      // Java: this.mapobj.put("colorFilter", 4105);
      self.mapobj.insert(hash_str("colorFilter"), 4105);
      // Java: this.mapobj.put("iconFilter", 4106);
      self.mapobj.insert(hash_str("iconFilter"), 4106);
      // Java: this.mapobj.put("colId", 4107);
      self.mapobj.insert(hash_str("colId"), 4107);
      // Java: this.mapobj.put("hiddenButton", 4108);
      self.mapobj.insert(hash_str("hiddenButton"), 4108);
      // Java: this.mapobj.put("showButton", 4109);
      self.mapobj.insert(hash_str("showButton"), 4109);
      // Java: this.mapobj.put("filter", 4110);
      self.mapobj.insert(hash_str("filter"), 4110);
      // Java: this.mapobj.put("dateGroupItem", 4111);
      self.mapobj.insert(hash_str("dateGroupItem"), 4111);
      // Java: this.mapobj.put("blank", 4112);
      self.mapobj.insert(hash_str("blank"), 4112);
      // Java: this.mapobj.put("calendarType", 4113);
      self.mapobj.insert(hash_str("calendarType"), 4113);
      // Java: this.mapobj.put("val", 4114);
      self.mapobj.insert(hash_str("val"), 4114);
      // Java: this.mapobj.put("customFilter", 4115);
      self.mapobj.insert(hash_str("customFilter"), 4115);
      // Java: this.mapobj.put("and", 4116);
      self.mapobj.insert(hash_str("and"), 4116);
      // Java: this.mapobj.put("operator", 4117);
      self.mapobj.insert(hash_str("operator"), 4117);
      // Java: this.mapobj.put("top", 4118);
      self.mapobj.insert(hash_str("top"), 4118);
      // Java: this.mapobj.put("percent", 4119);
      self.mapobj.insert(hash_str("percent"), 4119);
      // Java: this.mapobj.put("filterVal", 4120);
      self.mapobj.insert(hash_str("filterVal"), 4120);
      // Java: this.mapobj.put("dxfId", 4121);
      self.mapobj.insert(hash_str("dxfId"), 4121);
      // Java: this.mapobj.put("cellColor", 4122);
      self.mapobj.insert(hash_str("cellColor"), 4122);
      // Java: this.mapobj.put("iconSet", 4123);
      self.mapobj.insert(hash_str("iconSet"), 4123);
      // Java: this.mapobj.put("iconId", 4124);
      self.mapobj.insert(hash_str("iconId"), 4124);
      // Java: this.mapobj.put("type", 4125);
      self.mapobj.insert(hash_str("type"), 4125);
      // Java: this.mapobj.put("valIso", 4126);
      self.mapobj.insert(hash_str("valIso"), 4126);
      // Java: this.mapobj.put("maxVal", 4127);
      self.mapobj.insert(hash_str("maxVal"), 4127);
      // Java: this.mapobj.put("maxValIso", 4128);
      self.mapobj.insert(hash_str("maxValIso"), 4128);
      // Java: this.mapobj.put("sortCondition", 4129);
      self.mapobj.insert(hash_str("sortCondition"), 4129);
      // Java: this.mapobj.put("columnSort", 4130);
      self.mapobj.insert(hash_str("columnSort"), 4130);
      // Java: this.mapobj.put("caseSensitive", 4131);
      self.mapobj.insert(hash_str("caseSensitive"), 4131);
      // Java: this.mapobj.put("sortMethod", 4132);
      self.mapobj.insert(hash_str("sortMethod"), 4132);
      // Java: this.mapobj.put("descending", 4133);
      self.mapobj.insert(hash_str("descending"), 4133);
      // Java: this.mapobj.put("sortBy", 4134);
      self.mapobj.insert(hash_str("sortBy"), 4134);
      // Java: this.mapobj.put("customList", 4135);
      self.mapobj.insert(hash_str("customList"), 4135);
      // Java: this.mapobj.put("year", 4136);
      self.mapobj.insert(hash_str("year"), 4136);
      // Java: this.mapobj.put("month", 4137);
      self.mapobj.insert(hash_str("month"), 4137);
      // Java: this.mapobj.put("day", 4138);
      self.mapobj.insert(hash_str("day"), 4138);
      // Java: this.mapobj.put("hour", 4139);
      self.mapobj.insert(hash_str("hour"), 4139);
      // Java: this.mapobj.put("minute", 4140);
      self.mapobj.insert(hash_str("minute"), 4140);
      // Java: this.mapobj.put("second", 4141);
      self.mapobj.insert(hash_str("second"), 4141);
      // Java: this.mapobj.put("dateTimeGrouping", 4142);
      self.mapobj.insert(hash_str("dateTimeGrouping"), 4142);
      // Java: this.mapobj.put("v", 4143);
      self.mapobj.insert(hash_str("v"), 4143);
      // Java: this.mapobj.put("uri", 4144);
      self.mapobj.insert(hash_str("uri"), 4144);
      // Java: this.mapobj.put("moveWithCells", 4145);
      self.mapobj.insert(hash_str("moveWithCells"), 4145);
      // Java: this.mapobj.put("sizeWithCells", 4146);
      self.mapobj.insert(hash_str("sizeWithCells"), 4146);
      // Java: this.mapobj.put("z-order", 4147);
      self.mapobj.insert(hash_str("z-order"), 4147);
      // Java: this.mapobj.put("ext", 4148);
      self.mapobj.insert(hash_str("ext"), 4148);
      // Java: this.mapobj.put("calcChain", 4149);
      self.mapobj.insert(hash_str("calcChain"), 4149);
      // Java: this.mapobj.put("c", 4150);
      self.mapobj.insert(hash_str("c"), 4150);
      // Java: this.mapobj.put("r", 4151);
      self.mapobj.insert(hash_str("r"), 4151);
      // Java: this.mapobj.put("i", 4152);
      self.mapobj.insert(hash_str("i"), 4152);
      // Java: this.mapobj.put("s", 4153);
      self.mapobj.insert(hash_str("s"), 4153);
      // Java: this.mapobj.put("l", 4154);
      self.mapobj.insert(hash_str("l"), 4154);
      // Java: this.mapobj.put("t", 4155);
      self.mapobj.insert(hash_str("t"), 4155);
      // Java: this.mapobj.put("a", 4156);
      self.mapobj.insert(hash_str("a"), 4156);
      // Java: this.mapobj.put("comments", 4157);
      self.mapobj.insert(hash_str("comments"), 4157);
      // Java: this.mapobj.put("authors", 4158);
      self.mapobj.insert(hash_str("authors"), 4158);
      // Java: this.mapobj.put("commentList", 4159);
      self.mapobj.insert(hash_str("commentList"), 4159);
      // Java: this.mapobj.put("author", 4160);
      self.mapobj.insert(hash_str("author"), 4160);
      // Java: this.mapobj.put("comment", 4161);
      self.mapobj.insert(hash_str("comment"), 4161);
      // Java: this.mapobj.put("text", 4162);
      self.mapobj.insert(hash_str("text"), 4162);
      // Java: this.mapobj.put("commentPr", 4163);
      self.mapobj.insert(hash_str("commentPr"), 4163);
      // Java: this.mapobj.put("authorId", 4164);
      self.mapobj.insert(hash_str("authorId"), 4164);
      // Java: this.mapobj.put("guid", 4165);
      self.mapobj.insert(hash_str("guid"), 4165);
      // Java: this.mapobj.put("shapeId", 4166);
      self.mapobj.insert(hash_str("shapeId"), 4166);
      // Java: this.mapobj.put("anchor", 4167);
      self.mapobj.insert(hash_str("anchor"), 4167);
      // Java: this.mapobj.put("locked", 4168);
      self.mapobj.insert(hash_str("locked"), 4168);
      // Java: this.mapobj.put("defaultSize", 4169);
      self.mapobj.insert(hash_str("defaultSize"), 4169);
      // Java: this.mapobj.put("print", 4170);
      self.mapobj.insert(hash_str("print"), 4170);
      // Java: this.mapobj.put("disabled", 4171);
      self.mapobj.insert(hash_str("disabled"), 4171);
      // Java: this.mapobj.put("uiObject", 4172);
      self.mapobj.insert(hash_str("uiObject"), 4172);
      // Java: this.mapobj.put("autoFill", 4173);
      self.mapobj.insert(hash_str("autoFill"), 4173);
      // Java: this.mapobj.put("autoLine", 4174);
      self.mapobj.insert(hash_str("autoLine"), 4174);
      // Java: this.mapobj.put("altText", 4175);
      self.mapobj.insert(hash_str("altText"), 4175);
      // Java: this.mapobj.put("textHAlign", 4176);
      self.mapobj.insert(hash_str("textHAlign"), 4176);
      // Java: this.mapobj.put("textVAlign", 4177);
      self.mapobj.insert(hash_str("textVAlign"), 4177);
      // Java: this.mapobj.put("lockText", 4178);
      self.mapobj.insert(hash_str("lockText"), 4178);
      // Java: this.mapobj.put("justLastX", 4179);
      self.mapobj.insert(hash_str("justLastX"), 4179);
      // Java: this.mapobj.put("autoScale", 4180);
      self.mapobj.insert(hash_str("autoScale"), 4180);
      // Java: this.mapobj.put("rowHidden", 4181);
      self.mapobj.insert(hash_str("rowHidden"), 4181);
      // Java: this.mapobj.put("colHidden", 4182);
      self.mapobj.insert(hash_str("colHidden"), 4182);
      // Java: this.mapobj.put("MapInfo", 4183);
      self.mapobj.insert(hash_str("MapInfo"), 4183);
      // Java: this.mapobj.put("Schema", 4184);
      self.mapobj.insert(hash_str("Schema"), 4184);
      // Java: this.mapobj.put("Map", 4185);
      self.mapobj.insert(hash_str("Map"), 4185);
      // Java: this.mapobj.put("SelectionNamespaces", 4186);
      self.mapobj.insert(hash_str("SelectionNamespaces"), 4186);
      // Java: this.mapobj.put("ID", 4187);
      self.mapobj.insert(hash_str("ID"), 4187);
      // Java: this.mapobj.put("SchemaRef", 4188);
      self.mapobj.insert(hash_str("SchemaRef"), 4188);
      // Java: this.mapobj.put("Namespace", 4189);
      self.mapobj.insert(hash_str("Namespace"), 4189);
      // Java: this.mapobj.put("SchemaLanguage", 4190);
      self.mapobj.insert(hash_str("SchemaLanguage"), 4190);
      // Java: this.mapobj.put("DataBinding", 4191);
      self.mapobj.insert(hash_str("DataBinding"), 4191);
      // Java: this.mapobj.put("Name", 4192);
      self.mapobj.insert(hash_str("Name"), 4192);
      // Java: this.mapobj.put("RootElement", 4193);
      self.mapobj.insert(hash_str("RootElement"), 4193);
      // Java: this.mapobj.put("SchemaID", 4194);
      self.mapobj.insert(hash_str("SchemaID"), 4194);
      // Java: this.mapobj.put("ShowImportExportValidationErrors", 4195);
      self.mapobj.insert(hash_str("ShowImportExportValidationErrors"), 4195);
      // Java: this.mapobj.put("AutoFit", 4196);
      self.mapobj.insert(hash_str("AutoFit"), 4196);
      // Java: this.mapobj.put("Append", 4197);
      self.mapobj.insert(hash_str("Append"), 4197);
      // Java: this.mapobj.put("PreserveSortAFLayout", 4198);
      self.mapobj.insert(hash_str("PreserveSortAFLayout"), 4198);
      // Java: this.mapobj.put("PreserveFormat", 4199);
      self.mapobj.insert(hash_str("PreserveFormat"), 4199);
      // Java: this.mapobj.put("DataBindingName", 4200);
      self.mapobj.insert(hash_str("DataBindingName"), 4200);
      // Java: this.mapobj.put("FileBinding", 4201);
      self.mapobj.insert(hash_str("FileBinding"), 4201);
      // Java: this.mapobj.put("ConnectionID", 4202);
      self.mapobj.insert(hash_str("ConnectionID"), 4202);
      // Java: this.mapobj.put("FileBindingName", 4203);
      self.mapobj.insert(hash_str("FileBindingName"), 4203);
      // Java: this.mapobj.put("DataBindingLoadMode", 4204);
      self.mapobj.insert(hash_str("DataBindingLoadMode"), 4204);
      // Java: this.mapobj.put("connections", 4205);
      self.mapobj.insert(hash_str("connections"), 4205);
      // Java: this.mapobj.put("connection", 4206);
      self.mapobj.insert(hash_str("connection"), 4206);
      // Java: this.mapobj.put("dbPr", 4207);
      self.mapobj.insert(hash_str("dbPr"), 4207);
      // Java: this.mapobj.put("olapPr", 4208);
      self.mapobj.insert(hash_str("olapPr"), 4208);
      // Java: this.mapobj.put("webPr", 4209);
      self.mapobj.insert(hash_str("webPr"), 4209);
      // Java: this.mapobj.put("textPr", 4210);
      self.mapobj.insert(hash_str("textPr"), 4210);
      // Java: this.mapobj.put("parameters", 4211);
      self.mapobj.insert(hash_str("parameters"), 4211);
      // Java: this.mapobj.put("id", 4212);
      self.mapobj.insert(hash_str("id"), 4212);
      // Java: this.mapobj.put("sourceFile", 4213);
      self.mapobj.insert(hash_str("sourceFile"), 4213);
      // Java: this.mapobj.put("odcFile", 4214);
      self.mapobj.insert(hash_str("odcFile"), 4214);
      // Java: this.mapobj.put("keepAlive", 4215);
      self.mapobj.insert(hash_str("keepAlive"), 4215);
      // Java: this.mapobj.put("interval", 4216);
      self.mapobj.insert(hash_str("interval"), 4216);
      // Java: this.mapobj.put("name", 4217);
      self.mapobj.insert(hash_str("name"), 4217);
      // Java: this.mapobj.put("description", 4218);
      self.mapobj.insert(hash_str("description"), 4218);
      // Java: this.mapobj.put("reconnectionMethod", 4219);
      self.mapobj.insert(hash_str("reconnectionMethod"), 4219);
      // Java: this.mapobj.put("refreshedVersion", 4220);
      self.mapobj.insert(hash_str("refreshedVersion"), 4220);
      // Java: this.mapobj.put("minRefreshableVersion", 4221);
      self.mapobj.insert(hash_str("minRefreshableVersion"), 4221);
      // Java: this.mapobj.put("savePassword", 4222);
      self.mapobj.insert(hash_str("savePassword"), 4222);
      // Java: this.mapobj.put("new", 4223);
      self.mapobj.insert(hash_str("new"), 4223);
      // Java: this.mapobj.put("deleted", 4224);
      self.mapobj.insert(hash_str("deleted"), 4224);
      // Java: this.mapobj.put("onlyUseConnectionFile", 4225);
      self.mapobj.insert(hash_str("onlyUseConnectionFile"), 4225);
      // Java: this.mapobj.put("background", 4226);
      self.mapobj.insert(hash_str("background"), 4226);
      // Java: this.mapobj.put("refreshOnLoad", 4227);
      self.mapobj.insert(hash_str("refreshOnLoad"), 4227);
      // Java: this.mapobj.put("saveData", 4228);
      self.mapobj.insert(hash_str("saveData"), 4228);
      // Java: this.mapobj.put("credentials", 4229);
      self.mapobj.insert(hash_str("credentials"), 4229);
      // Java: this.mapobj.put("singleSignOnId", 4230);
      self.mapobj.insert(hash_str("singleSignOnId"), 4230);
      // Java: this.mapobj.put("command", 4231);
      self.mapobj.insert(hash_str("command"), 4231);
      // Java: this.mapobj.put("serverCommand", 4232);
      self.mapobj.insert(hash_str("serverCommand"), 4232);
      // Java: this.mapobj.put("commandType", 4233);
      self.mapobj.insert(hash_str("commandType"), 4233);
      // Java: this.mapobj.put("local", 4234);
      self.mapobj.insert(hash_str("local"), 4234);
      // Java: this.mapobj.put("localConnection", 4235);
      self.mapobj.insert(hash_str("localConnection"), 4235);
      // Java: this.mapobj.put("localRefresh", 4236);
      self.mapobj.insert(hash_str("localRefresh"), 4236);
      // Java: this.mapobj.put("sendLocale", 4237);
      self.mapobj.insert(hash_str("sendLocale"), 4237);
      // Java: this.mapobj.put("rowDrillCount", 4238);
      self.mapobj.insert(hash_str("rowDrillCount"), 4238);
      // Java: this.mapobj.put("serverFill", 4239);
      self.mapobj.insert(hash_str("serverFill"), 4239);
      // Java: this.mapobj.put("serverNumberFormat", 4240);
      self.mapobj.insert(hash_str("serverNumberFormat"), 4240);
      // Java: this.mapobj.put("serverFont", 4241);
      self.mapobj.insert(hash_str("serverFont"), 4241);
      // Java: this.mapobj.put("serverFontColor", 4242);
      self.mapobj.insert(hash_str("serverFontColor"), 4242);
      // Java: this.mapobj.put("tables", 4243);
      self.mapobj.insert(hash_str("tables"), 4243);
      // Java: this.mapobj.put("xml", 4244);
      self.mapobj.insert(hash_str("xml"), 4244);
      // Java: this.mapobj.put("sourceData", 4245);
      self.mapobj.insert(hash_str("sourceData"), 4245);
      // Java: this.mapobj.put("parsePre", 4246);
      self.mapobj.insert(hash_str("parsePre"), 4246);
      // Java: this.mapobj.put("consecutive", 4247);
      self.mapobj.insert(hash_str("consecutive"), 4247);
      // Java: this.mapobj.put("firstRow", 4248);
      self.mapobj.insert(hash_str("firstRow"), 4248);
      // Java: this.mapobj.put("xl97", 4249);
      self.mapobj.insert(hash_str("xl97"), 4249);
      // Java: this.mapobj.put("textDates", 4250);
      self.mapobj.insert(hash_str("textDates"), 4250);
      // Java: this.mapobj.put("xl2000", 4251);
      self.mapobj.insert(hash_str("xl2000"), 4251);
      // Java: this.mapobj.put("url", 4252);
      self.mapobj.insert(hash_str("url"), 4252);
      // Java: this.mapobj.put("post", 4253);
      self.mapobj.insert(hash_str("post"), 4253);
      // Java: this.mapobj.put("htmlTables", 4254);
      self.mapobj.insert(hash_str("htmlTables"), 4254);
      // Java: this.mapobj.put("htmlFormat", 4255);
      self.mapobj.insert(hash_str("htmlFormat"), 4255);
      // Java: this.mapobj.put("editPage", 4256);
      self.mapobj.insert(hash_str("editPage"), 4256);
      // Java: this.mapobj.put("parameter", 4257);
      self.mapobj.insert(hash_str("parameter"), 4257);
      // Java: this.mapobj.put("count", 4258);
      self.mapobj.insert(hash_str("count"), 4258);
      // Java: this.mapobj.put("sqlType", 4259);
      self.mapobj.insert(hash_str("sqlType"), 4259);
      // Java: this.mapobj.put("parameterType", 4260);
      self.mapobj.insert(hash_str("parameterType"), 4260);
      // Java: this.mapobj.put("refreshOnChange", 4261);
      self.mapobj.insert(hash_str("refreshOnChange"), 4261);
      // Java: this.mapobj.put("prompt", 4262);
      self.mapobj.insert(hash_str("prompt"), 4262);
      // Java: this.mapobj.put("boolean", 4263);
      self.mapobj.insert(hash_str("boolean"), 4263);
      // Java: this.mapobj.put("double", 4264);
      self.mapobj.insert(hash_str("double"), 4264);
      // Java: this.mapobj.put("integer", 4265);
      self.mapobj.insert(hash_str("integer"), 4265);
      // Java: this.mapobj.put("string", 4266);
      self.mapobj.insert(hash_str("string"), 4266);
      // Java: this.mapobj.put("cell", 4267);
      self.mapobj.insert(hash_str("cell"), 4267);
      // Java: this.mapobj.put("m", 4268);
      self.mapobj.insert(hash_str("m"), 4268);
      // Java: this.mapobj.put("x", 4269);
      self.mapobj.insert(hash_str("x"), 4269);
      // Java: this.mapobj.put("textFields", 4270);
      self.mapobj.insert(hash_str("textFields"), 4270);
      // Java: this.mapobj.put("fileType", 4271);
      self.mapobj.insert(hash_str("fileType"), 4271);
      // Java: this.mapobj.put("codePage", 4272);
      self.mapobj.insert(hash_str("codePage"), 4272);
      // Java: this.mapobj.put("characterSet", 4273);
      self.mapobj.insert(hash_str("characterSet"), 4273);
      // Java: this.mapobj.put("delimited", 4274);
      self.mapobj.insert(hash_str("delimited"), 4274);
      // Java: this.mapobj.put("decimal", 4275);
      self.mapobj.insert(hash_str("decimal"), 4275);
      // Java: this.mapobj.put("thousands", 4276);
      self.mapobj.insert(hash_str("thousands"), 4276);
      // Java: this.mapobj.put("tab", 4277);
      self.mapobj.insert(hash_str("tab"), 4277);
      // Java: this.mapobj.put("space", 4278);
      self.mapobj.insert(hash_str("space"), 4278);
      // Java: this.mapobj.put("comma", 4279);
      self.mapobj.insert(hash_str("comma"), 4279);
      // Java: this.mapobj.put("semicolon", 4280);
      self.mapobj.insert(hash_str("semicolon"), 4280);
      // Java: this.mapobj.put("qualifier", 4281);
      self.mapobj.insert(hash_str("qualifier"), 4281);
      // Java: this.mapobj.put("delimiter", 4282);
      self.mapobj.insert(hash_str("delimiter"), 4282);
      // Java: this.mapobj.put("textField", 4283);
      self.mapobj.insert(hash_str("textField"), 4283);
      // Java: this.mapobj.put("position", 4284);
      self.mapobj.insert(hash_str("position"), 4284);
      // Java: this.mapobj.put("pivotCacheDefinition", 4285);
      self.mapobj.insert(hash_str("pivotCacheDefinition"), 4285);
      // Java: this.mapobj.put("pivotCacheRecords", 4286);
      self.mapobj.insert(hash_str("pivotCacheRecords"), 4286);
      // Java: this.mapobj.put("pivotTableDefinition", 4287);
      self.mapobj.insert(hash_str("pivotTableDefinition"), 4287);
      // Java: this.mapobj.put("cacheSource", 4288);
      self.mapobj.insert(hash_str("cacheSource"), 4288);
      // Java: this.mapobj.put("cacheFields", 4289);
      self.mapobj.insert(hash_str("cacheFields"), 4289);
      // Java: this.mapobj.put("cacheHierarchies", 4290);
      self.mapobj.insert(hash_str("cacheHierarchies"), 4290);
      // Java: this.mapobj.put("kpis", 4291);
      self.mapobj.insert(hash_str("kpis"), 4291);
      // Java: this.mapobj.put("tupleCache", 4292);
      self.mapobj.insert(hash_str("tupleCache"), 4292);
      // Java: this.mapobj.put("calculatedItems", 4293);
      self.mapobj.insert(hash_str("calculatedItems"), 4293);
      // Java: this.mapobj.put("calculatedMembers", 4294);
      self.mapobj.insert(hash_str("calculatedMembers"), 4294);
      // Java: this.mapobj.put("dimensions", 4295);
      self.mapobj.insert(hash_str("dimensions"), 4295);
      // Java: this.mapobj.put("measureGroups", 4296);
      self.mapobj.insert(hash_str("measureGroups"), 4296);
      // Java: this.mapobj.put("maps", 4297);
      self.mapobj.insert(hash_str("maps"), 4297);
      // Java: this.mapobj.put("invalid", 4298);
      self.mapobj.insert(hash_str("invalid"), 4298);
      // Java: this.mapobj.put("optimizeMemory", 4299);
      self.mapobj.insert(hash_str("optimizeMemory"), 4299);
      // Java: this.mapobj.put("enableRefresh", 4300);
      self.mapobj.insert(hash_str("enableRefresh"), 4300);
      // Java: this.mapobj.put("refreshedBy", 4301);
      self.mapobj.insert(hash_str("refreshedBy"), 4301);
      // Java: this.mapobj.put("refreshedDate", 4302);
      self.mapobj.insert(hash_str("refreshedDate"), 4302);
      // Java: this.mapobj.put("refreshedDateIso", 4303);
      self.mapobj.insert(hash_str("refreshedDateIso"), 4303);
      // Java: this.mapobj.put("backgroundQuery", 4304);
      self.mapobj.insert(hash_str("backgroundQuery"), 4304);
      // Java: this.mapobj.put("missingItemsLimit", 4305);
      self.mapobj.insert(hash_str("missingItemsLimit"), 4305);
      // Java: this.mapobj.put("createdVersion", 4306);
      self.mapobj.insert(hash_str("createdVersion"), 4306);
      // Java: this.mapobj.put("recordCount", 4307);
      self.mapobj.insert(hash_str("recordCount"), 4307);
      // Java: this.mapobj.put("upgradeOnRefresh", 4308);
      self.mapobj.insert(hash_str("upgradeOnRefresh"), 4308);
      // Java: this.mapobj.put("supportSubquery", 4309);
      self.mapobj.insert(hash_str("supportSubquery"), 4309);
      // Java: this.mapobj.put("supportAdvancedDrill", 4310);
      self.mapobj.insert(hash_str("supportAdvancedDrill"), 4310);
      // Java: this.mapobj.put("cacheField", 4311);
      self.mapobj.insert(hash_str("cacheField"), 4311);
      // Java: this.mapobj.put("sharedItems", 4312);
      self.mapobj.insert(hash_str("sharedItems"), 4312);
      // Java: this.mapobj.put("fieldGroup", 4313);
      self.mapobj.insert(hash_str("fieldGroup"), 4313);
      // Java: this.mapobj.put("mpMap", 4314);
      self.mapobj.insert(hash_str("mpMap"), 4314);
      // Java: this.mapobj.put("caption", 4315);
      self.mapobj.insert(hash_str("caption"), 4315);
      // Java: this.mapobj.put("propertyName", 4316);
      self.mapobj.insert(hash_str("propertyName"), 4316);
      // Java: this.mapobj.put("serverField", 4317);
      self.mapobj.insert(hash_str("serverField"), 4317);
      // Java: this.mapobj.put("uniqueList", 4318);
      self.mapobj.insert(hash_str("uniqueList"), 4318);
      // Java: this.mapobj.put("numFmtId", 4319);
      self.mapobj.insert(hash_str("numFmtId"), 4319);
      // Java: this.mapobj.put("formula", 4320);
      self.mapobj.insert(hash_str("formula"), 4320);
      // Java: this.mapobj.put("hierarchy", 4321);
      self.mapobj.insert(hash_str("hierarchy"), 4321);
      // Java: this.mapobj.put("level", 4322);
      self.mapobj.insert(hash_str("level"), 4322);
      // Java: this.mapobj.put("databaseField", 4323);
      self.mapobj.insert(hash_str("databaseField"), 4323);
      // Java: this.mapobj.put("mappingCount", 4324);
      self.mapobj.insert(hash_str("mappingCount"), 4324);
      // Java: this.mapobj.put("memberPropertyField", 4325);
      self.mapobj.insert(hash_str("memberPropertyField"), 4325);
      // Java: this.mapobj.put("worksheetSource", 4326);
      self.mapobj.insert(hash_str("worksheetSource"), 4326);
      // Java: this.mapobj.put("consolidation", 4327);
      self.mapobj.insert(hash_str("consolidation"), 4327);
      // Java: this.mapobj.put("connectionId", 4328);
      self.mapobj.insert(hash_str("connectionId"), 4328);
      // Java: this.mapobj.put("sheet", 4329);
      self.mapobj.insert(hash_str("sheet"), 4329);
      // Java: this.mapobj.put("pages", 4330);
      self.mapobj.insert(hash_str("pages"), 4330);
      // Java: this.mapobj.put("rangeSets", 4331);
      self.mapobj.insert(hash_str("rangeSets"), 4331);
      // Java: this.mapobj.put("autoPage", 4332);
      self.mapobj.insert(hash_str("autoPage"), 4332);
      // Java: this.mapobj.put("page", 4333);
      self.mapobj.insert(hash_str("page"), 4333);
      // Java: this.mapobj.put("pageItem", 4334);
      self.mapobj.insert(hash_str("pageItem"), 4334);
      // Java: this.mapobj.put("rangeSet", 4335);
      self.mapobj.insert(hash_str("rangeSet"), 4335);
      // Java: this.mapobj.put("i1", 4336);
      self.mapobj.insert(hash_str("i1"), 4336);
      // Java: this.mapobj.put("i2", 4337);
      self.mapobj.insert(hash_str("i2"), 4337);
      // Java: this.mapobj.put("i3", 4338);
      self.mapobj.insert(hash_str("i3"), 4338);
      // Java: this.mapobj.put("i4", 4339);
      self.mapobj.insert(hash_str("i4"), 4339);
      // Java: this.mapobj.put("n", 4340);
      self.mapobj.insert(hash_str("n"), 4340);
      // Java: this.mapobj.put("b", 4341);
      self.mapobj.insert(hash_str("b"), 4341);
      // Java: this.mapobj.put("e", 4342);
      self.mapobj.insert(hash_str("e"), 4342);
      // Java: this.mapobj.put("d", 4343);
      self.mapobj.insert(hash_str("d"), 4343);
      // Java: this.mapobj.put("containsSemiMixedTypes", 4344);
      self.mapobj.insert(hash_str("containsSemiMixedTypes"), 4344);
      // Java: this.mapobj.put("containsNonDate", 4345);
      self.mapobj.insert(hash_str("containsNonDate"), 4345);
      // Java: this.mapobj.put("containsDate", 4346);
      self.mapobj.insert(hash_str("containsDate"), 4346);
      // Java: this.mapobj.put("containsString", 4347);
      self.mapobj.insert(hash_str("containsString"), 4347);
      // Java: this.mapobj.put("containsBlank", 4348);
      self.mapobj.insert(hash_str("containsBlank"), 4348);
      // Java: this.mapobj.put("containsMixedTypes", 4349);
      self.mapobj.insert(hash_str("containsMixedTypes"), 4349);
      // Java: this.mapobj.put("containsNumber", 4350);
      self.mapobj.insert(hash_str("containsNumber"), 4350);
      // Java: this.mapobj.put("containsInteger", 4351);
      self.mapobj.insert(hash_str("containsInteger"), 4351);
      // Java: this.mapobj.put("minValue", 4352);
      self.mapobj.insert(hash_str("minValue"), 4352);
      // Java: this.mapobj.put("maxValue", 4353);
      self.mapobj.insert(hash_str("maxValue"), 4353);
      // Java: this.mapobj.put("minDate", 4354);
      self.mapobj.insert(hash_str("minDate"), 4354);
      // Java: this.mapobj.put("maxDate", 4355);
      self.mapobj.insert(hash_str("maxDate"), 4355);
      // Java: this.mapobj.put("longText", 4356);
      self.mapobj.insert(hash_str("longText"), 4356);
      // Java: this.mapobj.put("tpls", 4357);
      self.mapobj.insert(hash_str("tpls"), 4357);
      // Java: this.mapobj.put("u", 4358);
      self.mapobj.insert(hash_str("u"), 4358);
      // Java: this.mapobj.put("f", 4359);
      self.mapobj.insert(hash_str("f"), 4359);
      // Java: this.mapobj.put("cp", 4360);
      self.mapobj.insert(hash_str("cp"), 4360);
      // Java: this.mapobj.put("in", 4361);
      self.mapobj.insert(hash_str("in"), 4361);
      // Java: this.mapobj.put("bc", 4362);
      self.mapobj.insert(hash_str("bc"), 4362);
      // Java: this.mapobj.put("fc", 4363);
      self.mapobj.insert(hash_str("fc"), 4363);
      // Java: this.mapobj.put("un", 4364);
      self.mapobj.insert(hash_str("un"), 4364);
      // Java: this.mapobj.put("st", 4365);
      self.mapobj.insert(hash_str("st"), 4365);
      // Java: this.mapobj.put("rangePr", 4366);
      self.mapobj.insert(hash_str("rangePr"), 4366);
      // Java: this.mapobj.put("discretePr", 4367);
      self.mapobj.insert(hash_str("discretePr"), 4367);
      // Java: this.mapobj.put("groupItems", 4368);
      self.mapobj.insert(hash_str("groupItems"), 4368);
      // Java: this.mapobj.put("par", 4369);
      self.mapobj.insert(hash_str("par"), 4369);
      // Java: this.mapobj.put("base", 4370);
      self.mapobj.insert(hash_str("base"), 4370);
      // Java: this.mapobj.put("autoStart", 4371);
      self.mapobj.insert(hash_str("autoStart"), 4371);
      // Java: this.mapobj.put("autoEnd", 4372);
      self.mapobj.insert(hash_str("autoEnd"), 4372);
      // Java: this.mapobj.put("groupBy", 4373);
      self.mapobj.insert(hash_str("groupBy"), 4373);
      // Java: this.mapobj.put("startNum", 4374);
      self.mapobj.insert(hash_str("startNum"), 4374);
      // Java: this.mapobj.put("endNum", 4375);
      self.mapobj.insert(hash_str("endNum"), 4375);
      // Java: this.mapobj.put("startDate", 4376);
      self.mapobj.insert(hash_str("startDate"), 4376);
      // Java: this.mapobj.put("endDate", 4377);
      self.mapobj.insert(hash_str("endDate"), 4377);
      // Java: this.mapobj.put("groupInterval", 4378);
      self.mapobj.insert(hash_str("groupInterval"), 4378);
      // Java: this.mapobj.put("kpi", 4379);
      self.mapobj.insert(hash_str("kpi"), 4379);
      // Java: this.mapobj.put("uniqueName", 4380);
      self.mapobj.insert(hash_str("uniqueName"), 4380);
      // Java: this.mapobj.put("displayFolder", 4381);
      self.mapobj.insert(hash_str("displayFolder"), 4381);
      // Java: this.mapobj.put("measureGroup", 4382);
      self.mapobj.insert(hash_str("measureGroup"), 4382);
      // Java: this.mapobj.put("parent", 4383);
      self.mapobj.insert(hash_str("parent"), 4383);
      // Java: this.mapobj.put("value", 4384);
      self.mapobj.insert(hash_str("value"), 4384);
      // Java: this.mapobj.put("goal", 4385);
      self.mapobj.insert(hash_str("goal"), 4385);
      // Java: this.mapobj.put("status", 4386);
      self.mapobj.insert(hash_str("status"), 4386);
      // Java: this.mapobj.put("trend", 4387);
      self.mapobj.insert(hash_str("trend"), 4387);
      // Java: this.mapobj.put("weight", 4388);
      self.mapobj.insert(hash_str("weight"), 4388);
      // Java: this.mapobj.put("time", 4389);
      self.mapobj.insert(hash_str("time"), 4389);
      // Java: this.mapobj.put("cacheHierarchy", 4390);
      self.mapobj.insert(hash_str("cacheHierarchy"), 4390);
      // Java: this.mapobj.put("fieldsUsage", 4391);
      self.mapobj.insert(hash_str("fieldsUsage"), 4391);
      // Java: this.mapobj.put("groupLevels", 4392);
      self.mapobj.insert(hash_str("groupLevels"), 4392);
      // Java: this.mapobj.put("measure", 4393);
      self.mapobj.insert(hash_str("measure"), 4393);
      // Java: this.mapobj.put("set", 4394);
      self.mapobj.insert(hash_str("set"), 4394);
      // Java: this.mapobj.put("parentSet", 4395);
      self.mapobj.insert(hash_str("parentSet"), 4395);
      // Java: this.mapobj.put("attribute", 4396);
      self.mapobj.insert(hash_str("attribute"), 4396);
      // Java: this.mapobj.put("keyAttribute", 4397);
      self.mapobj.insert(hash_str("keyAttribute"), 4397);
      // Java: this.mapobj.put("defaultMemberUniqueName", 4398);
      self.mapobj.insert(hash_str("defaultMemberUniqueName"), 4398);
      // Java: this.mapobj.put("allUniqueName", 4399);
      self.mapobj.insert(hash_str("allUniqueName"), 4399);
      // Java: this.mapobj.put("allCaption", 4400);
      self.mapobj.insert(hash_str("allCaption"), 4400);
      // Java: this.mapobj.put("dimensionUniqueName", 4401);
      self.mapobj.insert(hash_str("dimensionUniqueName"), 4401);
      // Java: this.mapobj.put("measures", 4402);
      self.mapobj.insert(hash_str("measures"), 4402);
      // Java: this.mapobj.put("oneField", 4403);
      self.mapobj.insert(hash_str("oneField"), 4403);
      // Java: this.mapobj.put("memberValueDatatype", 4404);
      self.mapobj.insert(hash_str("memberValueDatatype"), 4404);
      // Java: this.mapobj.put("unbalanced", 4405);
      self.mapobj.insert(hash_str("unbalanced"), 4405);
      // Java: this.mapobj.put("unbalancedGroup", 4406);
      self.mapobj.insert(hash_str("unbalancedGroup"), 4406);
      // Java: this.mapobj.put("hidden", 4407);
      self.mapobj.insert(hash_str("hidden"), 4407);
      // Java: this.mapobj.put("fieldUsage", 4408);
      self.mapobj.insert(hash_str("fieldUsage"), 4408);
      // Java: this.mapobj.put("groupLevel", 4409);
      self.mapobj.insert(hash_str("groupLevel"), 4409);
      // Java: this.mapobj.put("groups", 4410);
      self.mapobj.insert(hash_str("groups"), 4410);
      // Java: this.mapobj.put("user", 4411);
      self.mapobj.insert(hash_str("user"), 4411);
      // Java: this.mapobj.put("customRollUp", 4412);
      self.mapobj.insert(hash_str("customRollUp"), 4412);
      // Java: this.mapobj.put("group", 4413);
      self.mapobj.insert(hash_str("group"), 4413);
      // Java: this.mapobj.put("groupMembers", 4414);
      self.mapobj.insert(hash_str("groupMembers"), 4414);
      // Java: this.mapobj.put("uniqueParent", 4415);
      self.mapobj.insert(hash_str("uniqueParent"), 4415);
      // Java: this.mapobj.put("groupMember", 4416);
      self.mapobj.insert(hash_str("groupMember"), 4416);
      // Java: this.mapobj.put("entries", 4417);
      self.mapobj.insert(hash_str("entries"), 4417);
      // Java: this.mapobj.put("sets", 4418);
      self.mapobj.insert(hash_str("sets"), 4418);
      // Java: this.mapobj.put("queryCache", 4419);
      self.mapobj.insert(hash_str("queryCache"), 4419);
      // Java: this.mapobj.put("serverFormats", 4420);
      self.mapobj.insert(hash_str("serverFormats"), 4420);
      // Java: this.mapobj.put("culture", 4421);
      self.mapobj.insert(hash_str("culture"), 4421);
      // Java: this.mapobj.put("format", 4422);
      self.mapobj.insert(hash_str("format"), 4422);
      // Java: this.mapobj.put("serverFormat", 4423);
      self.mapobj.insert(hash_str("serverFormat"), 4423);
      // Java: this.mapobj.put("tpl", 4424);
      self.mapobj.insert(hash_str("tpl"), 4424);
      // Java: this.mapobj.put("fld", 4425);
      self.mapobj.insert(hash_str("fld"), 4425);
      // Java: this.mapobj.put("hier", 4426);
      self.mapobj.insert(hash_str("hier"), 4426);
      // Java: this.mapobj.put("item", 4427);
      self.mapobj.insert(hash_str("item"), 4427);
      // Java: this.mapobj.put("sortByTuple", 4428);
      self.mapobj.insert(hash_str("sortByTuple"), 4428);
      // Java: this.mapobj.put("maxRank", 4429);
      self.mapobj.insert(hash_str("maxRank"), 4429);
      // Java: this.mapobj.put("setDefinition", 4430);
      self.mapobj.insert(hash_str("setDefinition"), 4430);
      // Java: this.mapobj.put("sortType", 4431);
      self.mapobj.insert(hash_str("sortType"), 4431);
      // Java: this.mapobj.put("queryFailed", 4432);
      self.mapobj.insert(hash_str("queryFailed"), 4432);
      // Java: this.mapobj.put("query", 4433);
      self.mapobj.insert(hash_str("query"), 4433);
      // Java: this.mapobj.put("mdx", 4434);
      self.mapobj.insert(hash_str("mdx"), 4434);
      // Java: this.mapobj.put("calculatedItem", 4435);
      self.mapobj.insert(hash_str("calculatedItem"), 4435);
      // Java: this.mapobj.put("pivotArea", 4436);
      self.mapobj.insert(hash_str("pivotArea"), 4436);
      // Java: this.mapobj.put("field", 4437);
      self.mapobj.insert(hash_str("field"), 4437);
      // Java: this.mapobj.put("calculatedMember", 4438);
      self.mapobj.insert(hash_str("calculatedMember"), 4438);
      // Java: this.mapobj.put("memberName", 4439);
      self.mapobj.insert(hash_str("memberName"), 4439);
      // Java: this.mapobj.put("solveOrder", 4440);
      self.mapobj.insert(hash_str("solveOrder"), 4440);
      // Java: this.mapobj.put("location", 4441);
      self.mapobj.insert(hash_str("location"), 4441);
      // Java: this.mapobj.put("pivotFields", 4442);
      self.mapobj.insert(hash_str("pivotFields"), 4442);
      // Java: this.mapobj.put("rowFields", 4443);
      self.mapobj.insert(hash_str("rowFields"), 4443);
      // Java: this.mapobj.put("rowItems", 4444);
      self.mapobj.insert(hash_str("rowItems"), 4444);
      // Java: this.mapobj.put("colFields", 4445);
      self.mapobj.insert(hash_str("colFields"), 4445);
      // Java: this.mapobj.put("colItems", 4446);
      self.mapobj.insert(hash_str("colItems"), 4446);
      // Java: this.mapobj.put("pageFields", 4447);
      self.mapobj.insert(hash_str("pageFields"), 4447);
      // Java: this.mapobj.put("dataFields", 4448);
      self.mapobj.insert(hash_str("dataFields"), 4448);
      // Java: this.mapobj.put("formats", 4449);
      self.mapobj.insert(hash_str("formats"), 4449);
      // Java: this.mapobj.put("conditionalFormats", 4450);
      self.mapobj.insert(hash_str("conditionalFormats"), 4450);
      // Java: this.mapobj.put("chartFormats", 4451);
      self.mapobj.insert(hash_str("chartFormats"), 4451);
      // Java: this.mapobj.put("pivotHierarchies", 4452);
      self.mapobj.insert(hash_str("pivotHierarchies"), 4452);
      // Java: this.mapobj.put("pivotTableStyleInfo", 4453);
      self.mapobj.insert(hash_str("pivotTableStyleInfo"), 4453);
      // Java: this.mapobj.put("rowHierarchiesUsage", 4454);
      self.mapobj.insert(hash_str("rowHierarchiesUsage"), 4454);
      // Java: this.mapobj.put("colHierarchiesUsage", 4455);
      self.mapobj.insert(hash_str("colHierarchiesUsage"), 4455);
      // Java: this.mapobj.put("cacheId", 4456);
      self.mapobj.insert(hash_str("cacheId"), 4456);
      // Java: this.mapobj.put("dataOnRows", 4457);
      self.mapobj.insert(hash_str("dataOnRows"), 4457);
      // Java: this.mapobj.put("dataPosition", 4458);
      self.mapobj.insert(hash_str("dataPosition"), 4458);
      // Java: this.mapobj.put("dataCaption", 4459);
      self.mapobj.insert(hash_str("dataCaption"), 4459);
      // Java: this.mapobj.put("grandTotalCaption", 4460);
      self.mapobj.insert(hash_str("grandTotalCaption"), 4460);
      // Java: this.mapobj.put("errorCaption", 4461);
      self.mapobj.insert(hash_str("errorCaption"), 4461);
      // Java: this.mapobj.put("showError", 4462);
      self.mapobj.insert(hash_str("showError"), 4462);
      // Java: this.mapobj.put("missingCaption", 4463);
      self.mapobj.insert(hash_str("missingCaption"), 4463);
      // Java: this.mapobj.put("showMissing", 4464);
      self.mapobj.insert(hash_str("showMissing"), 4464);
      // Java: this.mapobj.put("pageStyle", 4465);
      self.mapobj.insert(hash_str("pageStyle"), 4465);
      // Java: this.mapobj.put("pivotTableStyle", 4466);
      self.mapobj.insert(hash_str("pivotTableStyle"), 4466);
      // Java: this.mapobj.put("vacatedStyle", 4467);
      self.mapobj.insert(hash_str("vacatedStyle"), 4467);
      // Java: this.mapobj.put("tag", 4468);
      self.mapobj.insert(hash_str("tag"), 4468);
      // Java: this.mapobj.put("updatedVersion", 4469);
      self.mapobj.insert(hash_str("updatedVersion"), 4469);
      // Java: this.mapobj.put("asteriskTotals", 4470);
      self.mapobj.insert(hash_str("asteriskTotals"), 4470);
      // Java: this.mapobj.put("showItems", 4471);
      self.mapobj.insert(hash_str("showItems"), 4471);
      // Java: this.mapobj.put("editData", 4472);
      self.mapobj.insert(hash_str("editData"), 4472);
      // Java: this.mapobj.put("disableFieldList", 4473);
      self.mapobj.insert(hash_str("disableFieldList"), 4473);
      // Java: this.mapobj.put("showCalcMbrs", 4474);
      self.mapobj.insert(hash_str("showCalcMbrs"), 4474);
      // Java: this.mapobj.put("visualTotals", 4475);
      self.mapobj.insert(hash_str("visualTotals"), 4475);
      // Java: this.mapobj.put("showMultipleLabel", 4476);
      self.mapobj.insert(hash_str("showMultipleLabel"), 4476);
      // Java: this.mapobj.put("showDataDropDown", 4477);
      self.mapobj.insert(hash_str("showDataDropDown"), 4477);
      // Java: this.mapobj.put("showDrill", 4478);
      self.mapobj.insert(hash_str("showDrill"), 4478);
      // Java: this.mapobj.put("printDrill", 4479);
      self.mapobj.insert(hash_str("printDrill"), 4479);
      // Java: this.mapobj.put("showMemberPropertyTips", 4480);
      self.mapobj.insert(hash_str("showMemberPropertyTips"), 4480);
      // Java: this.mapobj.put("showDataTips", 4481);
      self.mapobj.insert(hash_str("showDataTips"), 4481);
      // Java: this.mapobj.put("enableWizard", 4482);
      self.mapobj.insert(hash_str("enableWizard"), 4482);
      // Java: this.mapobj.put("enableDrill", 4483);
      self.mapobj.insert(hash_str("enableDrill"), 4483);
      // Java: this.mapobj.put("enableFieldProperties", 4484);
      self.mapobj.insert(hash_str("enableFieldProperties"), 4484);
      // Java: this.mapobj.put("preserveFormatting", 4485);
      self.mapobj.insert(hash_str("preserveFormatting"), 4485);
      // Java: this.mapobj.put("useAutoFormatting", 4486);
      self.mapobj.insert(hash_str("useAutoFormatting"), 4486);
      // Java: this.mapobj.put("pageWrap", 4487);
      self.mapobj.insert(hash_str("pageWrap"), 4487);
      // Java: this.mapobj.put("pageOverThenDown", 4488);
      self.mapobj.insert(hash_str("pageOverThenDown"), 4488);
      // Java: this.mapobj.put("subtotalHiddenItems", 4489);
      self.mapobj.insert(hash_str("subtotalHiddenItems"), 4489);
      // Java: this.mapobj.put("rowGrandTotals", 4490);
      self.mapobj.insert(hash_str("rowGrandTotals"), 4490);
      // Java: this.mapobj.put("colGrandTotals", 4491);
      self.mapobj.insert(hash_str("colGrandTotals"), 4491);
      // Java: this.mapobj.put("fieldPrintTitles", 4492);
      self.mapobj.insert(hash_str("fieldPrintTitles"), 4492);
      // Java: this.mapobj.put("itemPrintTitles", 4493);
      self.mapobj.insert(hash_str("itemPrintTitles"), 4493);
      // Java: this.mapobj.put("mergeItem", 4494);
      self.mapobj.insert(hash_str("mergeItem"), 4494);
      // Java: this.mapobj.put("showDropZones", 4495);
      self.mapobj.insert(hash_str("showDropZones"), 4495);
      // Java: this.mapobj.put("indent", 4496);
      self.mapobj.insert(hash_str("indent"), 4496);
      // Java: this.mapobj.put("showEmptyRow", 4497);
      self.mapobj.insert(hash_str("showEmptyRow"), 4497);
      // Java: this.mapobj.put("showEmptyCol", 4498);
      self.mapobj.insert(hash_str("showEmptyCol"), 4498);
      // Java: this.mapobj.put("showHeaders", 4499);
      self.mapobj.insert(hash_str("showHeaders"), 4499);
      // Java: this.mapobj.put("compact", 4500);
      self.mapobj.insert(hash_str("compact"), 4500);
      // Java: this.mapobj.put("outline", 4501);
      self.mapobj.insert(hash_str("outline"), 4501);
      // Java: this.mapobj.put("outlineData", 4502);
      self.mapobj.insert(hash_str("outlineData"), 4502);
      // Java: this.mapobj.put("compactData", 4503);
      self.mapobj.insert(hash_str("compactData"), 4503);
      // Java: this.mapobj.put("published", 4504);
      self.mapobj.insert(hash_str("published"), 4504);
      // Java: this.mapobj.put("gridDropZones", 4505);
      self.mapobj.insert(hash_str("gridDropZones"), 4505);
      // Java: this.mapobj.put("immersive", 4506);
      self.mapobj.insert(hash_str("immersive"), 4506);
      // Java: this.mapobj.put("multipleFieldFilters", 4507);
      self.mapobj.insert(hash_str("multipleFieldFilters"), 4507);
      // Java: this.mapobj.put("chartFormat", 4508);
      self.mapobj.insert(hash_str("chartFormat"), 4508);
      // Java: this.mapobj.put("rowHeaderCaption", 4509);
      self.mapobj.insert(hash_str("rowHeaderCaption"), 4509);
      // Java: this.mapobj.put("colHeaderCaption", 4510);
      self.mapobj.insert(hash_str("colHeaderCaption"), 4510);
      // Java: this.mapobj.put("fieldListSortAscending", 4511);
      self.mapobj.insert(hash_str("fieldListSortAscending"), 4511);
      // Java: this.mapobj.put("mdxSubqueries", 4512);
      self.mapobj.insert(hash_str("mdxSubqueries"), 4512);
      // Java: this.mapobj.put("customListSort", 4513);
      self.mapobj.insert(hash_str("customListSort"), 4513);
      // Java: this.mapobj.put("firstHeaderRow", 4514);
      self.mapobj.insert(hash_str("firstHeaderRow"), 4514);
      // Java: this.mapobj.put("firstDataRow", 4515);
      self.mapobj.insert(hash_str("firstDataRow"), 4515);
      // Java: this.mapobj.put("firstDataCol", 4516);
      self.mapobj.insert(hash_str("firstDataCol"), 4516);
      // Java: this.mapobj.put("rowPageCount", 4517);
      self.mapobj.insert(hash_str("rowPageCount"), 4517);
      // Java: this.mapobj.put("colPageCount", 4518);
      self.mapobj.insert(hash_str("colPageCount"), 4518);
      // Java: this.mapobj.put("pivotField", 4519);
      self.mapobj.insert(hash_str("pivotField"), 4519);
      // Java: this.mapobj.put("items", 4520);
      self.mapobj.insert(hash_str("items"), 4520);
      // Java: this.mapobj.put("autoSortScope", 4521);
      self.mapobj.insert(hash_str("autoSortScope"), 4521);
      // Java: this.mapobj.put("axis", 4522);
      self.mapobj.insert(hash_str("axis"), 4522);
      // Java: this.mapobj.put("dataField", 4523);
      self.mapobj.insert(hash_str("dataField"), 4523);
      // Java: this.mapobj.put("subtotalCaption", 4524);
      self.mapobj.insert(hash_str("subtotalCaption"), 4524);
      // Java: this.mapobj.put("showDropDowns", 4525);
      self.mapobj.insert(hash_str("showDropDowns"), 4525);
      // Java: this.mapobj.put("hiddenLevel", 4526);
      self.mapobj.insert(hash_str("hiddenLevel"), 4526);
      // Java: this.mapobj.put("uniqueMemberProperty", 4527);
      self.mapobj.insert(hash_str("uniqueMemberProperty"), 4527);
      // Java: this.mapobj.put("allDrilled", 4528);
      self.mapobj.insert(hash_str("allDrilled"), 4528);
      // Java: this.mapobj.put("subtotalTop", 4529);
      self.mapobj.insert(hash_str("subtotalTop"), 4529);
      // Java: this.mapobj.put("dragToRow", 4530);
      self.mapobj.insert(hash_str("dragToRow"), 4530);
      // Java: this.mapobj.put("dragToCol", 4531);
      self.mapobj.insert(hash_str("dragToCol"), 4531);
      // Java: this.mapobj.put("multipleItemSelectionAllowed", 4532);
      self.mapobj.insert(hash_str("multipleItemSelectionAllowed"), 4532);
      // Java: this.mapobj.put("dragToPage", 4533);
      self.mapobj.insert(hash_str("dragToPage"), 4533);
      // Java: this.mapobj.put("dragToData", 4534);
      self.mapobj.insert(hash_str("dragToData"), 4534);
      // Java: this.mapobj.put("dragOff", 4535);
      self.mapobj.insert(hash_str("dragOff"), 4535);
      // Java: this.mapobj.put("showAll", 4536);
      self.mapobj.insert(hash_str("showAll"), 4536);
      // Java: this.mapobj.put("insertBlankRow", 4537);
      self.mapobj.insert(hash_str("insertBlankRow"), 4537);
      // Java: this.mapobj.put("insertPageBreak", 4538);
      self.mapobj.insert(hash_str("insertPageBreak"), 4538);
      // Java: this.mapobj.put("autoShow", 4539);
      self.mapobj.insert(hash_str("autoShow"), 4539);
      // Java: this.mapobj.put("topAutoShow", 4540);
      self.mapobj.insert(hash_str("topAutoShow"), 4540);
      // Java: this.mapobj.put("hideNewItems", 4541);
      self.mapobj.insert(hash_str("hideNewItems"), 4541);
      // Java: this.mapobj.put("measureFilter", 4542);
      self.mapobj.insert(hash_str("measureFilter"), 4542);
      // Java: this.mapobj.put("includeNewItemsInFilter", 4543);
      self.mapobj.insert(hash_str("includeNewItemsInFilter"), 4543);
      // Java: this.mapobj.put("itemPageCount", 4544);
      self.mapobj.insert(hash_str("itemPageCount"), 4544);
      // Java: this.mapobj.put("dataSourceSort", 4545);
      self.mapobj.insert(hash_str("dataSourceSort"), 4545);
      // Java: this.mapobj.put("nonAutoSortDefault", 4546);
      self.mapobj.insert(hash_str("nonAutoSortDefault"), 4546);
      // Java: this.mapobj.put("rankBy", 4547);
      self.mapobj.insert(hash_str("rankBy"), 4547);
      // Java: this.mapobj.put("defaultSubtotal", 4548);
      self.mapobj.insert(hash_str("defaultSubtotal"), 4548);
      // Java: this.mapobj.put("sumSubtotal", 4549);
      self.mapobj.insert(hash_str("sumSubtotal"), 4549);
      // Java: this.mapobj.put("countASubtotal", 4550);
      self.mapobj.insert(hash_str("countASubtotal"), 4550);
      // Java: this.mapobj.put("avgSubtotal", 4551);
      self.mapobj.insert(hash_str("avgSubtotal"), 4551);
      // Java: this.mapobj.put("maxSubtotal", 4552);
      self.mapobj.insert(hash_str("maxSubtotal"), 4552);
      // Java: this.mapobj.put("minSubtotal", 4553);
      self.mapobj.insert(hash_str("minSubtotal"), 4553);
      // Java: this.mapobj.put("productSubtotal", 4554);
      self.mapobj.insert(hash_str("productSubtotal"), 4554);
      // Java: this.mapobj.put("countSubtotal", 4555);
      self.mapobj.insert(hash_str("countSubtotal"), 4555);
      // Java: this.mapobj.put("stdDevSubtotal", 4556);
      self.mapobj.insert(hash_str("stdDevSubtotal"), 4556);
      // Java: this.mapobj.put("stdDevPSubtotal", 4557);
      self.mapobj.insert(hash_str("stdDevPSubtotal"), 4557);
      // Java: this.mapobj.put("varSubtotal", 4558);
      self.mapobj.insert(hash_str("varSubtotal"), 4558);
      // Java: this.mapobj.put("varPSubtotal", 4559);
      self.mapobj.insert(hash_str("varPSubtotal"), 4559);
      // Java: this.mapobj.put("showPropCell", 4560);
      self.mapobj.insert(hash_str("showPropCell"), 4560);
      // Java: this.mapobj.put("showPropTip", 4561);
      self.mapobj.insert(hash_str("showPropTip"), 4561);
      // Java: this.mapobj.put("showPropAsCaption", 4562);
      self.mapobj.insert(hash_str("showPropAsCaption"), 4562);
      // Java: this.mapobj.put("defaultAttributeDrillState", 4563);
      self.mapobj.insert(hash_str("defaultAttributeDrillState"), 4563);
      // Java: this.mapobj.put("h", 4564);
      self.mapobj.insert(hash_str("h"), 4564);
      // Java: this.mapobj.put("sd", 4565);
      self.mapobj.insert(hash_str("sd"), 4565);
      // Java: this.mapobj.put("pageField", 4566);
      self.mapobj.insert(hash_str("pageField"), 4566);
      // Java: this.mapobj.put("cap", 4567);
      self.mapobj.insert(hash_str("cap"), 4567);
      // Java: this.mapobj.put("subtotal", 4568);
      self.mapobj.insert(hash_str("subtotal"), 4568);
      // Java: this.mapobj.put("showDataAs", 4569);
      self.mapobj.insert(hash_str("showDataAs"), 4569);
      // Java: this.mapobj.put("baseField", 4570);
      self.mapobj.insert(hash_str("baseField"), 4570);
      // Java: this.mapobj.put("baseItem", 4571);
      self.mapobj.insert(hash_str("baseItem"), 4571);
      // Java: this.mapobj.put("action", 4572);
      self.mapobj.insert(hash_str("action"), 4572);
      // Java: this.mapobj.put("conditionalFormat", 4573);
      self.mapobj.insert(hash_str("conditionalFormat"), 4573);
      // Java: this.mapobj.put("pivotAreas", 4574);
      self.mapobj.insert(hash_str("pivotAreas"), 4574);
      // Java: this.mapobj.put("scope", 4575);
      self.mapobj.insert(hash_str("scope"), 4575);
      // Java: this.mapobj.put("priority", 4576);
      self.mapobj.insert(hash_str("priority"), 4576);
      // Java: this.mapobj.put("chart", 4577);
      self.mapobj.insert(hash_str("chart"), 4577);
      // Java: this.mapobj.put("series", 4578);
      self.mapobj.insert(hash_str("series"), 4578);
      // Java: this.mapobj.put("pivotHierarchy", 4579);
      self.mapobj.insert(hash_str("pivotHierarchy"), 4579);
      // Java: this.mapobj.put("mps", 4580);
      self.mapobj.insert(hash_str("mps"), 4580);
      // Java: this.mapobj.put("members", 4581);
      self.mapobj.insert(hash_str("members"), 4581);
      // Java: this.mapobj.put("showInFieldList", 4582);
      self.mapobj.insert(hash_str("showInFieldList"), 4582);
      // Java: this.mapobj.put("rowHierarchyUsage", 4583);
      self.mapobj.insert(hash_str("rowHierarchyUsage"), 4583);
      // Java: this.mapobj.put("colHierarchyUsage", 4584);
      self.mapobj.insert(hash_str("colHierarchyUsage"), 4584);
      // Java: this.mapobj.put("hierarchyUsage", 4585);
      self.mapobj.insert(hash_str("hierarchyUsage"), 4585);
      // Java: this.mapobj.put("mp", 4586);
      self.mapobj.insert(hash_str("mp"), 4586);
      // Java: this.mapobj.put("showCell", 4587);
      self.mapobj.insert(hash_str("showCell"), 4587);
      // Java: this.mapobj.put("showTip", 4588);
      self.mapobj.insert(hash_str("showTip"), 4588);
      // Java: this.mapobj.put("showAsCaption", 4589);
      self.mapobj.insert(hash_str("showAsCaption"), 4589);
      // Java: this.mapobj.put("nameLen", 4590);
      self.mapobj.insert(hash_str("nameLen"), 4590);
      // Java: this.mapobj.put("pPos", 4591);
      self.mapobj.insert(hash_str("pPos"), 4591);
      // Java: this.mapobj.put("pLen", 4592);
      self.mapobj.insert(hash_str("pLen"), 4592);
      // Java: this.mapobj.put("member", 4593);
      self.mapobj.insert(hash_str("member"), 4593);
      // Java: this.mapobj.put("dimension", 4594);
      self.mapobj.insert(hash_str("dimension"), 4594);
      // Java: this.mapobj.put("map", 4595);
      self.mapobj.insert(hash_str("map"), 4595);
      // Java: this.mapobj.put("showRowHeaders", 4596);
      self.mapobj.insert(hash_str("showRowHeaders"), 4596);
      // Java: this.mapobj.put("showColHeaders", 4597);
      self.mapobj.insert(hash_str("showColHeaders"), 4597);
      // Java: this.mapobj.put("showRowStripes", 4598);
      self.mapobj.insert(hash_str("showRowStripes"), 4598);
      // Java: this.mapobj.put("showColStripes", 4599);
      self.mapobj.insert(hash_str("showColStripes"), 4599);
      // Java: this.mapobj.put("showLastColumn", 4600);
      self.mapobj.insert(hash_str("showLastColumn"), 4600);
      // Java: this.mapobj.put("autoFilter", 4601);
      self.mapobj.insert(hash_str("autoFilter"), 4601);
      // Java: this.mapobj.put("mpFld", 4602);
      self.mapobj.insert(hash_str("mpFld"), 4602);
      // Java: this.mapobj.put("evalOrder", 4603);
      self.mapobj.insert(hash_str("evalOrder"), 4603);
      // Java: this.mapobj.put("iMeasureHier", 4604);
      self.mapobj.insert(hash_str("iMeasureHier"), 4604);
      // Java: this.mapobj.put("iMeasureFld", 4605);
      self.mapobj.insert(hash_str("iMeasureFld"), 4605);
      // Java: this.mapobj.put("stringValue1", 4606);
      self.mapobj.insert(hash_str("stringValue1"), 4606);
      // Java: this.mapobj.put("stringValue2", 4607);
      self.mapobj.insert(hash_str("stringValue2"), 4607);
      // Java: this.mapobj.put("references", 4608);
      self.mapobj.insert(hash_str("references"), 4608);
      // Java: this.mapobj.put("dataOnly", 4609);
      self.mapobj.insert(hash_str("dataOnly"), 4609);
      // Java: this.mapobj.put("labelOnly", 4610);
      self.mapobj.insert(hash_str("labelOnly"), 4610);
      // Java: this.mapobj.put("grandRow", 4611);
      self.mapobj.insert(hash_str("grandRow"), 4611);
      // Java: this.mapobj.put("grandCol", 4612);
      self.mapobj.insert(hash_str("grandCol"), 4612);
      // Java: this.mapobj.put("cacheIndex", 4613);
      self.mapobj.insert(hash_str("cacheIndex"), 4613);
      // Java: this.mapobj.put("offset", 4614);
      self.mapobj.insert(hash_str("offset"), 4614);
      // Java: this.mapobj.put("collapsedLevelsAreSubtotals", 4615);
      self.mapobj.insert(hash_str("collapsedLevelsAreSubtotals"), 4615);
      // Java: this.mapobj.put("fieldPosition", 4616);
      self.mapobj.insert(hash_str("fieldPosition"), 4616);
      // Java: this.mapobj.put("reference", 4617);
      self.mapobj.insert(hash_str("reference"), 4617);
      // Java: this.mapobj.put("selected", 4618);
      self.mapobj.insert(hash_str("selected"), 4618);
      // Java: this.mapobj.put("byPosition", 4619);
      self.mapobj.insert(hash_str("byPosition"), 4619);
      // Java: this.mapobj.put("relative", 4620);
      self.mapobj.insert(hash_str("relative"), 4620);
      // Java: this.mapobj.put("queryTable", 4621);
      self.mapobj.insert(hash_str("queryTable"), 4621);
      // Java: this.mapobj.put("queryTableRefresh", 4622);
      self.mapobj.insert(hash_str("queryTableRefresh"), 4622);
      // Java: this.mapobj.put("headers", 4623);
      self.mapobj.insert(hash_str("headers"), 4623);
      // Java: this.mapobj.put("rowNumbers", 4624);
      self.mapobj.insert(hash_str("rowNumbers"), 4624);
      // Java: this.mapobj.put("disableRefresh", 4625);
      self.mapobj.insert(hash_str("disableRefresh"), 4625);
      // Java: this.mapobj.put("backgroundRefresh", 4626);
      self.mapobj.insert(hash_str("backgroundRefresh"), 4626);
      // Java: this.mapobj.put("firstBackgroundRefresh", 4627);
      self.mapobj.insert(hash_str("firstBackgroundRefresh"), 4627);
      // Java: this.mapobj.put("growShrinkType", 4628);
      self.mapobj.insert(hash_str("growShrinkType"), 4628);
      // Java: this.mapobj.put("fillFormulas", 4629);
      self.mapobj.insert(hash_str("fillFormulas"), 4629);
      // Java: this.mapobj.put("removeDataOnSave", 4630);
      self.mapobj.insert(hash_str("removeDataOnSave"), 4630);
      // Java: this.mapobj.put("disableEdit", 4631);
      self.mapobj.insert(hash_str("disableEdit"), 4631);
      // Java: this.mapobj.put("adjustColumnWidth", 4632);
      self.mapobj.insert(hash_str("adjustColumnWidth"), 4632);
      // Java: this.mapobj.put("intermediate", 4633);
      self.mapobj.insert(hash_str("intermediate"), 4633);
      // Java: this.mapobj.put("queryTableFields", 4634);
      self.mapobj.insert(hash_str("queryTableFields"), 4634);
      // Java: this.mapobj.put("queryTableDeletedFields", 4635);
      self.mapobj.insert(hash_str("queryTableDeletedFields"), 4635);
      // Java: this.mapobj.put("preserveSortFilterLayout", 4636);
      self.mapobj.insert(hash_str("preserveSortFilterLayout"), 4636);
      // Java: this.mapobj.put("fieldIdWrapped", 4637);
      self.mapobj.insert(hash_str("fieldIdWrapped"), 4637);
      // Java: this.mapobj.put("headersInLastRefresh", 4638);
      self.mapobj.insert(hash_str("headersInLastRefresh"), 4638);
      // Java: this.mapobj.put("minimumVersion", 4639);
      self.mapobj.insert(hash_str("minimumVersion"), 4639);
      // Java: this.mapobj.put("nextId", 4640);
      self.mapobj.insert(hash_str("nextId"), 4640);
      // Java: this.mapobj.put("unboundColumnsLeft", 4641);
      self.mapobj.insert(hash_str("unboundColumnsLeft"), 4641);
      // Java: this.mapobj.put("unboundColumnsRight", 4642);
      self.mapobj.insert(hash_str("unboundColumnsRight"), 4642);
      // Java: this.mapobj.put("deletedField", 4643);
      self.mapobj.insert(hash_str("deletedField"), 4643);
      // Java: this.mapobj.put("queryTableField", 4644);
      self.mapobj.insert(hash_str("queryTableField"), 4644);
      // Java: this.mapobj.put("dataBound", 4645);
      self.mapobj.insert(hash_str("dataBound"), 4645);
      // Java: this.mapobj.put("clipped", 4646);
      self.mapobj.insert(hash_str("clipped"), 4646);
      // Java: this.mapobj.put("tableColumnId", 4647);
      self.mapobj.insert(hash_str("tableColumnId"), 4647);
      // Java: this.mapobj.put("sst", 4648);
      self.mapobj.insert(hash_str("sst"), 4648);
      // Java: this.mapobj.put("si", 4649);
      self.mapobj.insert(hash_str("si"), 4649);
      // Java: this.mapobj.put("uniqueCount", 4650);
      self.mapobj.insert(hash_str("uniqueCount"), 4650);
      // Java: this.mapobj.put("sb", 4651);
      self.mapobj.insert(hash_str("sb"), 4651);
      // Java: this.mapobj.put("eb", 4652);
      self.mapobj.insert(hash_str("eb"), 4652);
      // Java: this.mapobj.put("rPr", 4653);
      self.mapobj.insert(hash_str("rPr"), 4653);
      // Java: this.mapobj.put("rFont", 4654);
      self.mapobj.insert(hash_str("rFont"), 4654);
      // Java: this.mapobj.put("charset", 4655);
      self.mapobj.insert(hash_str("charset"), 4655);
      // Java: this.mapobj.put("family", 4656);
      self.mapobj.insert(hash_str("family"), 4656);
      // Java: this.mapobj.put("strike", 4657);
      self.mapobj.insert(hash_str("strike"), 4657);
      // Java: this.mapobj.put("shadow", 4658);
      self.mapobj.insert(hash_str("shadow"), 4658);
      // Java: this.mapobj.put("condense", 4659);
      self.mapobj.insert(hash_str("condense"), 4659);
      // Java: this.mapobj.put("extend", 4660);
      self.mapobj.insert(hash_str("extend"), 4660);
      // Java: this.mapobj.put("color", 4661);
      self.mapobj.insert(hash_str("color"), 4661);
      // Java: this.mapobj.put("sz", 4662);
      self.mapobj.insert(hash_str("sz"), 4662);
      // Java: this.mapobj.put("vertAlign", 4663);
      self.mapobj.insert(hash_str("vertAlign"), 4663);
      // Java: this.mapobj.put("scheme", 4664);
      self.mapobj.insert(hash_str("scheme"), 4664);
      // Java: this.mapobj.put("rPh", 4665);
      self.mapobj.insert(hash_str("rPh"), 4665);
      // Java: this.mapobj.put("phoneticPr", 4666);
      self.mapobj.insert(hash_str("phoneticPr"), 4666);
      // Java: this.mapobj.put("fontId", 4667);
      self.mapobj.insert(hash_str("fontId"), 4667);
      // Java: this.mapobj.put("alignment", 4668);
      self.mapobj.insert(hash_str("alignment"), 4668);
      // Java: this.mapobj.put("revisions", 4669);
      self.mapobj.insert(hash_str("revisions"), 4669);
      // Java: this.mapobj.put("header", 4670);
      self.mapobj.insert(hash_str("header"), 4670);
      // Java: this.mapobj.put("lastGuid", 4671);
      self.mapobj.insert(hash_str("lastGuid"), 4671);
      // Java: this.mapobj.put("shared", 4672);
      self.mapobj.insert(hash_str("shared"), 4672);
      // Java: this.mapobj.put("diskRevisions", 4673);
      self.mapobj.insert(hash_str("diskRevisions"), 4673);
      // Java: this.mapobj.put("history", 4674);
      self.mapobj.insert(hash_str("history"), 4674);
      // Java: this.mapobj.put("trackRevisions", 4675);
      self.mapobj.insert(hash_str("trackRevisions"), 4675);
      // Java: this.mapobj.put("exclusive", 4676);
      self.mapobj.insert(hash_str("exclusive"), 4676);
      // Java: this.mapobj.put("revisionId", 4677);
      self.mapobj.insert(hash_str("revisionId"), 4677);
      // Java: this.mapobj.put("version", 4678);
      self.mapobj.insert(hash_str("version"), 4678);
      // Java: this.mapobj.put("keepChangeHistory", 4679);
      self.mapobj.insert(hash_str("keepChangeHistory"), 4679);
      // Java: this.mapobj.put("protected", 4680);
      self.mapobj.insert(hash_str("protected"), 4680);
      // Java: this.mapobj.put("preserveHistory", 4681);
      self.mapobj.insert(hash_str("preserveHistory"), 4681);
      // Java: this.mapobj.put("rrc", 4682);
      self.mapobj.insert(hash_str("rrc"), 4682);
      // Java: this.mapobj.put("rm", 4683);
      self.mapobj.insert(hash_str("rm"), 4683);
      // Java: this.mapobj.put("rcv", 4684);
      self.mapobj.insert(hash_str("rcv"), 4684);
      // Java: this.mapobj.put("rsnm", 4685);
      self.mapobj.insert(hash_str("rsnm"), 4685);
      // Java: this.mapobj.put("ris", 4686);
      self.mapobj.insert(hash_str("ris"), 4686);
      // Java: this.mapobj.put("rcc", 4687);
      self.mapobj.insert(hash_str("rcc"), 4687);
      // Java: this.mapobj.put("rfmt", 4688);
      self.mapobj.insert(hash_str("rfmt"), 4688);
      // Java: this.mapobj.put("raf", 4689);
      self.mapobj.insert(hash_str("raf"), 4689);
      // Java: this.mapobj.put("rdn", 4690);
      self.mapobj.insert(hash_str("rdn"), 4690);
      // Java: this.mapobj.put("rcmt", 4691);
      self.mapobj.insert(hash_str("rcmt"), 4691);
      // Java: this.mapobj.put("rqt", 4692);
      self.mapobj.insert(hash_str("rqt"), 4692);
      // Java: this.mapobj.put("rcft", 4693);
      self.mapobj.insert(hash_str("rcft"), 4693);
      // Java: this.mapobj.put("rId", 4694);
      self.mapobj.insert(hash_str("rId"), 4694);
      // Java: this.mapobj.put("ua", 4695);
      self.mapobj.insert(hash_str("ua"), 4695);
      // Java: this.mapobj.put("ra", 4696);
      self.mapobj.insert(hash_str("ra"), 4696);
      // Java: this.mapobj.put("sheetIdMap", 4697);
      self.mapobj.insert(hash_str("sheetIdMap"), 4697);
      // Java: this.mapobj.put("reviewedList", 4698);
      self.mapobj.insert(hash_str("reviewedList"), 4698);
      // Java: this.mapobj.put("dateTime", 4699);
      self.mapobj.insert(hash_str("dateTime"), 4699);
      // Java: this.mapobj.put("maxSheetId", 4700);
      self.mapobj.insert(hash_str("maxSheetId"), 4700);
      // Java: this.mapobj.put("userName", 4701);
      self.mapobj.insert(hash_str("userName"), 4701);
      // Java: this.mapobj.put("minRId", 4702);
      self.mapobj.insert(hash_str("minRId"), 4702);
      // Java: this.mapobj.put("maxRId", 4703);
      self.mapobj.insert(hash_str("maxRId"), 4703);
      // Java: this.mapobj.put("sheetId", 4704);
      self.mapobj.insert(hash_str("sheetId"), 4704);
      // Java: this.mapobj.put("reviewed", 4705);
      self.mapobj.insert(hash_str("reviewed"), 4705);
      // Java: this.mapobj.put("index", 4706);
      self.mapobj.insert(hash_str("index"), 4706);
      // Java: this.mapobj.put("exp", 4707);
      self.mapobj.insert(hash_str("exp"), 4707);
      // Java: this.mapobj.put("ref3D", 4708);
      self.mapobj.insert(hash_str("ref3D"), 4708);
      // Java: this.mapobj.put("array", 4709);
      self.mapobj.insert(hash_str("array"), 4709);
      // Java: this.mapobj.put("nf", 4710);
      self.mapobj.insert(hash_str("nf"), 4710);
      // Java: this.mapobj.put("cs", 4711);
      self.mapobj.insert(hash_str("cs"), 4711);
      // Java: this.mapobj.put("dr", 4712);
      self.mapobj.insert(hash_str("dr"), 4712);
      // Java: this.mapobj.put("dn", 4713);
      self.mapobj.insert(hash_str("dn"), 4713);
      // Java: this.mapobj.put("sId", 4714);
      self.mapobj.insert(hash_str("sId"), 4714);
      // Java: this.mapobj.put("undo", 4715);
      self.mapobj.insert(hash_str("undo"), 4715);
      // Java: this.mapobj.put("eol", 4716);
      self.mapobj.insert(hash_str("eol"), 4716);
      // Java: this.mapobj.put("edge", 4717);
      self.mapobj.insert(hash_str("edge"), 4717);
      // Java: this.mapobj.put("source", 4718);
      self.mapobj.insert(hash_str("source"), 4718);
      // Java: this.mapobj.put("destination", 4719);
      self.mapobj.insert(hash_str("destination"), 4719);
      // Java: this.mapobj.put("sourceSheetId", 4720);
      self.mapobj.insert(hash_str("sourceSheetId"), 4720);
      // Java: this.mapobj.put("oldName", 4721);
      self.mapobj.insert(hash_str("oldName"), 4721);
      // Java: this.mapobj.put("newName", 4722);
      self.mapobj.insert(hash_str("newName"), 4722);
      // Java: this.mapobj.put("sheetPosition", 4723);
      self.mapobj.insert(hash_str("sheetPosition"), 4723);
      // Java: this.mapobj.put("oc", 4724);
      self.mapobj.insert(hash_str("oc"), 4724);
      // Java: this.mapobj.put("nc", 4725);
      self.mapobj.insert(hash_str("nc"), 4725);
      // Java: this.mapobj.put("odxf", 4726);
      self.mapobj.insert(hash_str("odxf"), 4726);
      // Java: this.mapobj.put("ndxf", 4727);
      self.mapobj.insert(hash_str("ndxf"), 4727);
      // Java: this.mapobj.put("xfDxf", 4728);
      self.mapobj.insert(hash_str("xfDxf"), 4728);
      // Java: this.mapobj.put("dxf", 4729);
      self.mapobj.insert(hash_str("dxf"), 4729);
      // Java: this.mapobj.put("quotePrefix", 4730);
      self.mapobj.insert(hash_str("quotePrefix"), 4730);
      // Java: this.mapobj.put("oldQuotePrefix", 4731);
      self.mapobj.insert(hash_str("oldQuotePrefix"), 4731);
      // Java: this.mapobj.put("ph", 4732);
      self.mapobj.insert(hash_str("ph"), 4732);
      // Java: this.mapobj.put("oldPh", 4733);
      self.mapobj.insert(hash_str("oldPh"), 4733);
      // Java: this.mapobj.put("endOfListFormulaUpdate", 4734);
      self.mapobj.insert(hash_str("endOfListFormulaUpdate"), 4734);
      // Java: this.mapobj.put("sqref", 4735);
      self.mapobj.insert(hash_str("sqref"), 4735);
      // Java: this.mapobj.put("start", 4736);
      self.mapobj.insert(hash_str("start"), 4736);
      // Java: this.mapobj.put("length", 4737);
      self.mapobj.insert(hash_str("length"), 4737);
      // Java: this.mapobj.put("alwaysShow", 4738);
      self.mapobj.insert(hash_str("alwaysShow"), 4738);
      // Java: this.mapobj.put("old", 4739);
      self.mapobj.insert(hash_str("old"), 4739);
      // Java: this.mapobj.put("hiddenRow", 4740);
      self.mapobj.insert(hash_str("hiddenRow"), 4740);
      // Java: this.mapobj.put("hiddenColumn", 4741);
      self.mapobj.insert(hash_str("hiddenColumn"), 4741);
      // Java: this.mapobj.put("oldLength", 4742);
      self.mapobj.insert(hash_str("oldLength"), 4742);
      // Java: this.mapobj.put("newLength", 4743);
      self.mapobj.insert(hash_str("newLength"), 4743);
      // Java: this.mapobj.put("oldFormula", 4744);
      self.mapobj.insert(hash_str("oldFormula"), 4744);
      // Java: this.mapobj.put("localSheetId", 4745);
      self.mapobj.insert(hash_str("localSheetId"), 4745);
      // Java: this.mapobj.put("customView", 4746);
      self.mapobj.insert(hash_str("customView"), 4746);
      // Java: this.mapobj.put("function", 4747);
      self.mapobj.insert(hash_str("function"), 4747);
      // Java: this.mapobj.put("oldFunction", 4748);
      self.mapobj.insert(hash_str("oldFunction"), 4748);
      // Java: this.mapobj.put("functionGroupId", 4749);
      self.mapobj.insert(hash_str("functionGroupId"), 4749);
      // Java: this.mapobj.put("oldFunctionGroupId", 4750);
      self.mapobj.insert(hash_str("oldFunctionGroupId"), 4750);
      // Java: this.mapobj.put("shortcutKey", 4751);
      self.mapobj.insert(hash_str("shortcutKey"), 4751);
      // Java: this.mapobj.put("oldShortcutKey", 4752);
      self.mapobj.insert(hash_str("oldShortcutKey"), 4752);
      // Java: this.mapobj.put("oldHidden", 4753);
      self.mapobj.insert(hash_str("oldHidden"), 4753);
      // Java: this.mapobj.put("customMenu", 4754);
      self.mapobj.insert(hash_str("customMenu"), 4754);
      // Java: this.mapobj.put("oldCustomMenu", 4755);
      self.mapobj.insert(hash_str("oldCustomMenu"), 4755);
      // Java: this.mapobj.put("oldDescription", 4756);
      self.mapobj.insert(hash_str("oldDescription"), 4756);
      // Java: this.mapobj.put("help", 4757);
      self.mapobj.insert(hash_str("help"), 4757);
      // Java: this.mapobj.put("oldHelp", 4758);
      self.mapobj.insert(hash_str("oldHelp"), 4758);
      // Java: this.mapobj.put("statusBar", 4759);
      self.mapobj.insert(hash_str("statusBar"), 4759);
      // Java: this.mapobj.put("oldStatusBar", 4760);
      self.mapobj.insert(hash_str("oldStatusBar"), 4760);
      // Java: this.mapobj.put("oldComment", 4761);
      self.mapobj.insert(hash_str("oldComment"), 4761);
      // Java: this.mapobj.put("fieldId", 4762);
      self.mapobj.insert(hash_str("fieldId"), 4762);
      // Java: this.mapobj.put("users", 4763);
      self.mapobj.insert(hash_str("users"), 4763);
      // Java: this.mapobj.put("userInfo", 4764);
      self.mapobj.insert(hash_str("userInfo"), 4764);
      // Java: this.mapobj.put("worksheet", 4765);
      self.mapobj.insert(hash_str("worksheet"), 4765);
      // Java: this.mapobj.put("chartsheet", 4766);
      self.mapobj.insert(hash_str("chartsheet"), 4766);
      // Java: this.mapobj.put("dialogsheet", 4767);
      self.mapobj.insert(hash_str("dialogsheet"), 4767);
      // Java: this.mapobj.put("sheetPr", 4768);
      self.mapobj.insert(hash_str("sheetPr"), 4768);
      // Java: this.mapobj.put("sheetViews", 4769);
      self.mapobj.insert(hash_str("sheetViews"), 4769);
      // Java: this.mapobj.put("sheetFormatPr", 4770);
      self.mapobj.insert(hash_str("sheetFormatPr"), 4770);
      // Java: this.mapobj.put("cols", 4771);
      self.mapobj.insert(hash_str("cols"), 4771);
      // Java: this.mapobj.put("sheetData", 4772);
      self.mapobj.insert(hash_str("sheetData"), 4772);
      // Java: this.mapobj.put("sheetProtection", 4773);
      self.mapobj.insert(hash_str("sheetProtection"), 4773);
      // Java: this.mapobj.put("dataConsolidate", 4774);
      self.mapobj.insert(hash_str("dataConsolidate"), 4774);
      // Java: this.mapobj.put("customSheetViews", 4775);
      self.mapobj.insert(hash_str("customSheetViews"), 4775);
      // Java: this.mapobj.put("conditionalFormattings", 5498);
      self.mapobj.insert(hash_str("conditionalFormattings"), 5498);
      // Java: this.mapobj.put("conditionalFormatting", 4776);
      self.mapobj.insert(hash_str("conditionalFormatting"), 4776);
      // Java: this.mapobj.put("printOptions", 4777);
      self.mapobj.insert(hash_str("printOptions"), 4777);
      // Java: this.mapobj.put("pageMargins", 4778);
      self.mapobj.insert(hash_str("pageMargins"), 4778);
      // Java: this.mapobj.put("pageSetup", 4779);
      self.mapobj.insert(hash_str("pageSetup"), 4779);
      // Java: this.mapobj.put("headerFooter", 4780);
      self.mapobj.insert(hash_str("headerFooter"), 4780);
      // Java: this.mapobj.put("rowBreaks", 4781);
      self.mapobj.insert(hash_str("rowBreaks"), 4781);
      // Java: this.mapobj.put("colBreaks", 4782);
      self.mapobj.insert(hash_str("colBreaks"), 4782);
      // Java: this.mapobj.put("customProperties", 4783);
      self.mapobj.insert(hash_str("customProperties"), 4783);
      // Java: this.mapobj.put("drawing", 4784);
      self.mapobj.insert(hash_str("drawing"), 4784);
      // Java: this.mapobj.put("legacyDrawing", 4785);
      self.mapobj.insert(hash_str("legacyDrawing"), 4785);
      // Java: this.mapobj.put("legacyDrawingHF", 4786);
      self.mapobj.insert(hash_str("legacyDrawingHF"), 4786);
      // Java: this.mapobj.put("drawingHF", 4787);
      self.mapobj.insert(hash_str("drawingHF"), 4787);
      // Java: this.mapobj.put("picture", 4788);
      self.mapobj.insert(hash_str("picture"), 4788);
      // Java: this.mapobj.put("oleObjects", 4789);
      self.mapobj.insert(hash_str("oleObjects"), 4789);
      // Java: this.mapobj.put("sheetCalcPr", 4790);
      self.mapobj.insert(hash_str("sheetCalcPr"), 4790);
      // Java: this.mapobj.put("protectedRanges", 4791);
      self.mapobj.insert(hash_str("protectedRanges"), 4791);
      // Java: this.mapobj.put("scenarios", 4792);
      self.mapobj.insert(hash_str("scenarios"), 4792);
      // Java: this.mapobj.put("mergeCells", 4793);
      self.mapobj.insert(hash_str("mergeCells"), 4793);
      // Java: this.mapobj.put("dataValidations", 4794);
      self.mapobj.insert(hash_str("dataValidations"), 4794);
      // Java: this.mapobj.put("hyperlinks", 4795);
      self.mapobj.insert(hash_str("hyperlinks"), 4795);
      // Java: this.mapobj.put("cellWatches", 4796);
      self.mapobj.insert(hash_str("cellWatches"), 4796);
      // Java: this.mapobj.put("ignoredErrors", 4797);
      self.mapobj.insert(hash_str("ignoredErrors"), 4797);
      // Java: this.mapobj.put("smartTags", 4798);
      self.mapobj.insert(hash_str("smartTags"), 4798);
      // Java: this.mapobj.put("controls", 4799);
      self.mapobj.insert(hash_str("controls"), 4799);
      // Java: this.mapobj.put("webPublishItems", 4800);
      self.mapobj.insert(hash_str("webPublishItems"), 4800);
      // Java: this.mapobj.put("tableParts", 4801);
      self.mapobj.insert(hash_str("tableParts"), 4801);
      // Java: this.mapobj.put("row", 4802);
      self.mapobj.insert(hash_str("row"), 4802);
      // Java: this.mapobj.put("fullCalcOnLoad", 4803);
      self.mapobj.insert(hash_str("fullCalcOnLoad"), 4803);
      // Java: this.mapobj.put("baseColWidth", 4804);
      self.mapobj.insert(hash_str("baseColWidth"), 4804);
      // Java: this.mapobj.put("defaultColWidth", 4805);
      self.mapobj.insert(hash_str("defaultColWidth"), 4805);
      // Java: this.mapobj.put("defaultRowHeight", 4806);
      self.mapobj.insert(hash_str("defaultRowHeight"), 4806);
      // Java: this.mapobj.put("customHeight", 4807);
      self.mapobj.insert(hash_str("customHeight"), 4807);
      // Java: this.mapobj.put("zeroHeight", 4808);
      self.mapobj.insert(hash_str("zeroHeight"), 4808);
      // Java: this.mapobj.put("thickTop", 4809);
      self.mapobj.insert(hash_str("thickTop"), 4809);
      // Java: this.mapobj.put("thickBottom", 4810);
      self.mapobj.insert(hash_str("thickBottom"), 4810);
      // Java: this.mapobj.put("outlineLevelRow", 4811);
      self.mapobj.insert(hash_str("outlineLevelRow"), 4811);
      // Java: this.mapobj.put("outlineLevelCol", 4812);
      self.mapobj.insert(hash_str("outlineLevelCol"), 4812);
      // Java: this.mapobj.put("col", 4813);
      self.mapobj.insert(hash_str("col"), 4813);
      // Java: this.mapobj.put("min", 4814);
      self.mapobj.insert(hash_str("min"), 4814);
      // Java: this.mapobj.put("max", 4815);
      self.mapobj.insert(hash_str("max"), 4815);
      // Java: this.mapobj.put("width", 4816);
      self.mapobj.insert(hash_str("width"), 4816);
      // Java: this.mapobj.put("style", 4817);
      self.mapobj.insert(hash_str("style"), 4817);
      // Java: this.mapobj.put("bestFit", 4818);
      self.mapobj.insert(hash_str("bestFit"), 4818);
      // Java: this.mapobj.put("customWidth", 4819);
      self.mapobj.insert(hash_str("customWidth"), 4819);
      // Java: this.mapobj.put("phonetic", 4820);
      self.mapobj.insert(hash_str("phonetic"), 4820);
      // Java: this.mapobj.put("outlineLevel", 4821);
      self.mapobj.insert(hash_str("outlineLevel"), 4821);
      // Java: this.mapobj.put("collapsed", 4822);
      self.mapobj.insert(hash_str("collapsed"), 4822);
      // Java: this.mapobj.put("spans", 4823);
      self.mapobj.insert(hash_str("spans"), 4823);
      // Java: this.mapobj.put("customFormat", 4824);
      self.mapobj.insert(hash_str("customFormat"), 4824);
      // Java: this.mapobj.put("ht", 4825);
      self.mapobj.insert(hash_str("ht"), 4825);
      // Java: this.mapobj.put("thickBot", 4826);
      self.mapobj.insert(hash_str("thickBot"), 4826);
      // Java: this.mapobj.put("is", 4827);
      self.mapobj.insert(hash_str("is"), 4827);
      // Java: this.mapobj.put("cm", 4828);
      self.mapobj.insert(hash_str("cm"), 4828);
      // Java: this.mapobj.put("vm", 4829);
      self.mapobj.insert(hash_str("vm"), 4829);
      // Java: this.mapobj.put("tabColor", 4830);
      self.mapobj.insert(hash_str("tabColor"), 4830);
      // Java: this.mapobj.put("outlinePr", 4831);
      self.mapobj.insert(hash_str("outlinePr"), 4831);
      // Java: this.mapobj.put("pageSetUpPr", 4832);
      self.mapobj.insert(hash_str("pageSetUpPr"), 4832);
      // Java: this.mapobj.put("syncHorizontal", 4833);
      self.mapobj.insert(hash_str("syncHorizontal"), 4833);
      // Java: this.mapobj.put("syncVertical", 4834);
      self.mapobj.insert(hash_str("syncVertical"), 4834);
      // Java: this.mapobj.put("syncRef", 4835);
      self.mapobj.insert(hash_str("syncRef"), 4835);
      // Java: this.mapobj.put("transitionEvaluation", 4836);
      self.mapobj.insert(hash_str("transitionEvaluation"), 4836);
      // Java: this.mapobj.put("transitionEntry", 4837);
      self.mapobj.insert(hash_str("transitionEntry"), 4837);
      // Java: this.mapobj.put("codeName", 4838);
      self.mapobj.insert(hash_str("codeName"), 4838);
      // Java: this.mapobj.put("filterMode", 4839);
      self.mapobj.insert(hash_str("filterMode"), 4839);
      // Java: this.mapobj.put("enableFormatConditionsCalculation", 4840);
      self.mapobj.insert(hash_str("enableFormatConditionsCalculation"), 4840);
      // Java: this.mapobj.put("sheetView", 4841);
      self.mapobj.insert(hash_str("sheetView"), 4841);
      // Java: this.mapobj.put("pane", 4842);
      self.mapobj.insert(hash_str("pane"), 4842);
      // Java: this.mapobj.put("selection", 4843);
      self.mapobj.insert(hash_str("selection"), 4843);
      // Java: this.mapobj.put("pivotSelection", 4844);
      self.mapobj.insert(hash_str("pivotSelection"), 4844);
      // Java: this.mapobj.put("windowProtection", 4845);
      self.mapobj.insert(hash_str("windowProtection"), 4845);
      // Java: this.mapobj.put("showFormulas", 4846);
      self.mapobj.insert(hash_str("showFormulas"), 4846);
      // Java: this.mapobj.put("showGridLines", 4847);
      self.mapobj.insert(hash_str("showGridLines"), 4847);
      // Java: this.mapobj.put("showRowColHeaders", 4848);
      self.mapobj.insert(hash_str("showRowColHeaders"), 4848);
      // Java: this.mapobj.put("showZeros", 4849);
      self.mapobj.insert(hash_str("showZeros"), 4849);
      // Java: this.mapobj.put("rightToLeft", 4850);
      self.mapobj.insert(hash_str("rightToLeft"), 4850);
      // Java: this.mapobj.put("tabSelected", 4851);
      self.mapobj.insert(hash_str("tabSelected"), 4851);
      // Java: this.mapobj.put("showRuler", 4852);
      self.mapobj.insert(hash_str("showRuler"), 4852);
      // Java: this.mapobj.put("showOutlineSymbols", 4853);
      self.mapobj.insert(hash_str("showOutlineSymbols"), 4853);
      // Java: this.mapobj.put("defaultGridColor", 4854);
      self.mapobj.insert(hash_str("defaultGridColor"), 4854);
      // Java: this.mapobj.put("showWhiteSpace", 4855);
      self.mapobj.insert(hash_str("showWhiteSpace"), 4855);
      // Java: this.mapobj.put("view", 4856);
      self.mapobj.insert(hash_str("view"), 4856);
      // Java: this.mapobj.put("topLeftCell", 4857);
      self.mapobj.insert(hash_str("topLeftCell"), 4857);
      // Java: this.mapobj.put("colorId", 4858);
      self.mapobj.insert(hash_str("colorId"), 4858);
      // Java: this.mapobj.put("zoomScale", 4859);
      self.mapobj.insert(hash_str("zoomScale"), 4859);
      // Java: this.mapobj.put("zoomScaleNormal", 4860);
      self.mapobj.insert(hash_str("zoomScaleNormal"), 4860);
      // Java: this.mapobj.put("zoomScaleSheetLayoutView", 4861);
      self.mapobj.insert(hash_str("zoomScaleSheetLayoutView"), 4861);
      // Java: this.mapobj.put("zoomScalePageLayoutView", 4862);
      self.mapobj.insert(hash_str("zoomScalePageLayoutView"), 4862);
      // Java: this.mapobj.put("workbookViewId", 4863);
      self.mapobj.insert(hash_str("workbookViewId"), 4863);
      // Java: this.mapobj.put("xSplit", 4864);
      self.mapobj.insert(hash_str("xSplit"), 4864);
      // Java: this.mapobj.put("ySplit", 4865);
      self.mapobj.insert(hash_str("ySplit"), 4865);
      // Java: this.mapobj.put("activePane", 4866);
      self.mapobj.insert(hash_str("activePane"), 4866);
      // Java: this.mapobj.put("state", 4867);
      self.mapobj.insert(hash_str("state"), 4867);
      // Java: this.mapobj.put("showHeader", 4868);
      self.mapobj.insert(hash_str("showHeader"), 4868);
      // Java: this.mapobj.put("label", 4869);
      self.mapobj.insert(hash_str("label"), 4869);
      // Java: this.mapobj.put("data", 4870);
      self.mapobj.insert(hash_str("data"), 4870);
      // Java: this.mapobj.put("extendable", 4871);
      self.mapobj.insert(hash_str("extendable"), 4871);
      // Java: this.mapobj.put("activeRow", 4872);
      self.mapobj.insert(hash_str("activeRow"), 4872);
      // Java: this.mapobj.put("activeCol", 4873);
      self.mapobj.insert(hash_str("activeCol"), 4873);
      // Java: this.mapobj.put("previousRow", 4874);
      self.mapobj.insert(hash_str("previousRow"), 4874);
      // Java: this.mapobj.put("previousCol", 4875);
      self.mapobj.insert(hash_str("previousCol"), 4875);
      // Java: this.mapobj.put("click", 4876);
      self.mapobj.insert(hash_str("click"), 4876);
      // Java: this.mapobj.put("activeCell", 4877);
      self.mapobj.insert(hash_str("activeCell"), 4877);
      // Java: this.mapobj.put("activeCellId", 4878);
      self.mapobj.insert(hash_str("activeCellId"), 4878);
      // Java: this.mapobj.put("brk", 4879);
      self.mapobj.insert(hash_str("brk"), 4879);
      // Java: this.mapobj.put("manualBreakCount", 4880);
      self.mapobj.insert(hash_str("manualBreakCount"), 4880);
      // Java: this.mapobj.put("man", 4881);
      self.mapobj.insert(hash_str("man"), 4881);
      // Java: this.mapobj.put("pt", 4882);
      self.mapobj.insert(hash_str("pt"), 4882);
      // Java: this.mapobj.put("applyStyles", 4883);
      self.mapobj.insert(hash_str("applyStyles"), 4883);
      // Java: this.mapobj.put("summaryBelow", 4884);
      self.mapobj.insert(hash_str("summaryBelow"), 4884);
      // Java: this.mapobj.put("summaryRight", 4885);
      self.mapobj.insert(hash_str("summaryRight"), 4885);
      // Java: this.mapobj.put("autoPageBreaks", 4886);
      self.mapobj.insert(hash_str("autoPageBreaks"), 4886);
      // Java: this.mapobj.put("fitToPage", 4887);
      self.mapobj.insert(hash_str("fitToPage"), 4887);
      // Java: this.mapobj.put("dataRefs", 4888);
      self.mapobj.insert(hash_str("dataRefs"), 4888);
      // Java: this.mapobj.put("startLabels", 4889);
      self.mapobj.insert(hash_str("startLabels"), 4889);
      // Java: this.mapobj.put("leftLabels", 4890);
      self.mapobj.insert(hash_str("leftLabels"), 4890);
      // Java: this.mapobj.put("topLabels", 4891);
      self.mapobj.insert(hash_str("topLabels"), 4891);
      // Java: this.mapobj.put("link", 4892);
      self.mapobj.insert(hash_str("link"), 4892);
      // Java: this.mapobj.put("dataRef", 4893);
      self.mapobj.insert(hash_str("dataRef"), 4893);
      // Java: this.mapobj.put("mergeCell", 4894);
      self.mapobj.insert(hash_str("mergeCell"), 4894);
      // Java: this.mapobj.put("cellSmartTags", 4895);
      self.mapobj.insert(hash_str("cellSmartTags"), 4895);
      // Java: this.mapobj.put("cellSmartTag", 4896);
      self.mapobj.insert(hash_str("cellSmartTag"), 4896);
      // Java: this.mapobj.put("cellSmartTagPr", 4897);
      self.mapobj.insert(hash_str("cellSmartTagPr"), 4897);
      // Java: this.mapobj.put("xmlBased", 4898);
      self.mapobj.insert(hash_str("xmlBased"), 4898);
      // Java: this.mapobj.put("key", 4899);
      self.mapobj.insert(hash_str("key"), 4899);
      // Java: this.mapobj.put("lho", 4900);
      self.mapobj.insert(hash_str("lho"), 4900);
      // Java: this.mapobj.put("lhe", 4901);
      self.mapobj.insert(hash_str("lhe"), 4901);
      // Java: this.mapobj.put("lhf", 4902);
      self.mapobj.insert(hash_str("lhf"), 4902);
      // Java: this.mapobj.put("cho", 4903);
      self.mapobj.insert(hash_str("cho"), 4903);
      // Java: this.mapobj.put("che", 4904);
      self.mapobj.insert(hash_str("che"), 4904);
      // Java: this.mapobj.put("chf", 4905);
      self.mapobj.insert(hash_str("chf"), 4905);
      // Java: this.mapobj.put("rho", 4906);
      self.mapobj.insert(hash_str("rho"), 4906);
      // Java: this.mapobj.put("rhe", 4907);
      self.mapobj.insert(hash_str("rhe"), 4907);
      // Java: this.mapobj.put("rhf", 4908);
      self.mapobj.insert(hash_str("rhf"), 4908);
      // Java: this.mapobj.put("lfo", 4909);
      self.mapobj.insert(hash_str("lfo"), 4909);
      // Java: this.mapobj.put("lfe", 4910);
      self.mapobj.insert(hash_str("lfe"), 4910);
      // Java: this.mapobj.put("lff", 4911);
      self.mapobj.insert(hash_str("lff"), 4911);
      // Java: this.mapobj.put("cfo", 4912);
      self.mapobj.insert(hash_str("cfo"), 4912);
      // Java: this.mapobj.put("cfe", 4913);
      self.mapobj.insert(hash_str("cfe"), 4913);
      // Java: this.mapobj.put("cff", 4914);
      self.mapobj.insert(hash_str("cff"), 4914);
      // Java: this.mapobj.put("rfo", 4915);
      self.mapobj.insert(hash_str("rfo"), 4915);
      // Java: this.mapobj.put("rfe", 4916);
      self.mapobj.insert(hash_str("rfe"), 4916);
      // Java: this.mapobj.put("rff", 4917);
      self.mapobj.insert(hash_str("rff"), 4917);
      // Java: this.mapobj.put("customSheetView", 4918);
      self.mapobj.insert(hash_str("customSheetView"), 4918);
      // Java: this.mapobj.put("scale", 4919);
      self.mapobj.insert(hash_str("scale"), 4919);
      // Java: this.mapobj.put("showPageBreaks", 4920);
      self.mapobj.insert(hash_str("showPageBreaks"), 4920);
      // Java: this.mapobj.put("showRowCol", 4921);
      self.mapobj.insert(hash_str("showRowCol"), 4921);
      // Java: this.mapobj.put("outlineSymbols", 4922);
      self.mapobj.insert(hash_str("outlineSymbols"), 4922);
      // Java: this.mapobj.put("zeroValues", 4923);
      self.mapobj.insert(hash_str("zeroValues"), 4923);
      // Java: this.mapobj.put("printArea", 4924);
      self.mapobj.insert(hash_str("printArea"), 4924);
      // Java: this.mapobj.put("showAutoFilter", 4925);
      self.mapobj.insert(hash_str("showAutoFilter"), 4925);
      // Java: this.mapobj.put("hiddenRows", 4926);
      self.mapobj.insert(hash_str("hiddenRows"), 4926);
      // Java: this.mapobj.put("hiddenColumns", 4927);
      self.mapobj.insert(hash_str("hiddenColumns"), 4927);
      // Java: this.mapobj.put("filterUnique", 4928);
      self.mapobj.insert(hash_str("filterUnique"), 4928);
      // Java: this.mapobj.put("dataValidation", 4929);
      self.mapobj.insert(hash_str("dataValidation"), 4929);
      // Java: this.mapobj.put("disablePrompts", 4930);
      self.mapobj.insert(hash_str("disablePrompts"), 4930);
      // Java: this.mapobj.put("xWindow", 4931);
      self.mapobj.insert(hash_str("xWindow"), 4931);
      // Java: this.mapobj.put("yWindow", 4932);
      self.mapobj.insert(hash_str("yWindow"), 4932);
      // Java: this.mapobj.put("formula1", 4933);
      self.mapobj.insert(hash_str("formula1"), 4933);
      // Java: this.mapobj.put("formula2", 4934);
      self.mapobj.insert(hash_str("formula2"), 4934);
      // Java: this.mapobj.put("errorStyle", 4935);
      self.mapobj.insert(hash_str("errorStyle"), 4935);
      // Java: this.mapobj.put("imeMode", 4936);
      self.mapobj.insert(hash_str("imeMode"), 4936);
      // Java: this.mapobj.put("allowBlank", 4937);
      self.mapobj.insert(hash_str("allowBlank"), 4937);
      // Java: this.mapobj.put("showDropDown", 4938);
      self.mapobj.insert(hash_str("showDropDown"), 4938);
      // Java: this.mapobj.put("showInputMessage", 4939);
      self.mapobj.insert(hash_str("showInputMessage"), 4939);
      // Java: this.mapobj.put("showErrorMessage", 4940);
      self.mapobj.insert(hash_str("showErrorMessage"), 4940);
      // Java: this.mapobj.put("errorTitle", 4941);
      self.mapobj.insert(hash_str("errorTitle"), 4941);
      // Java: this.mapobj.put("error", 4942);
      self.mapobj.insert(hash_str("error"), 4942);
      // Java: this.mapobj.put("promptTitle", 4943);
      self.mapobj.insert(hash_str("promptTitle"), 4943);
      // Java: this.mapobj.put("cfRule", 4944);
      self.mapobj.insert(hash_str("cfRule"), 4944);
      // Java: this.mapobj.put("pivot", 4945);
      self.mapobj.insert(hash_str("pivot"), 4945);
      // Java: this.mapobj.put("colorScale", 4946);
      self.mapobj.insert(hash_str("colorScale"), 4946);
      // Java: this.mapobj.put("dataBar", 4947);
      self.mapobj.insert(hash_str("dataBar"), 4947);
      // Java: this.mapobj.put("stopIfTrue", 4948);
      self.mapobj.insert(hash_str("stopIfTrue"), 4948);
      // Java: this.mapobj.put("aboveAverage", 4949);
      self.mapobj.insert(hash_str("aboveAverage"), 4949);
      // Java: this.mapobj.put("bottom", 4950);
      self.mapobj.insert(hash_str("bottom"), 4950);
      // Java: this.mapobj.put("timePeriod", 4951);
      self.mapobj.insert(hash_str("timePeriod"), 4951);
      // Java: this.mapobj.put("rank", 4952);
      self.mapobj.insert(hash_str("rank"), 4952);
      // Java: this.mapobj.put("stdDev", 4953);
      self.mapobj.insert(hash_str("stdDev"), 4953);
      // Java: this.mapobj.put("equalAverage", 4954);
      self.mapobj.insert(hash_str("equalAverage"), 4954);
      // Java: this.mapobj.put("hyperlink", 4955);
      self.mapobj.insert(hash_str("hyperlink"), 4955);
      // Java: this.mapobj.put("tooltip", 4956);
      self.mapobj.insert(hash_str("tooltip"), 4956);
      // Java: this.mapobj.put("display", 4957);
      self.mapobj.insert(hash_str("display"), 4957);
      // Java: this.mapobj.put("aca", 4958);
      self.mapobj.insert(hash_str("aca"), 4958);
      // Java: this.mapobj.put("dt2D", 4959);
      self.mapobj.insert(hash_str("dt2D"), 4959);
      // Java: this.mapobj.put("dtr", 4960);
      self.mapobj.insert(hash_str("dtr"), 4960);
      // Java: this.mapobj.put("del1", 4961);
      self.mapobj.insert(hash_str("del1"), 4961);
      // Java: this.mapobj.put("del2", 4962);
      self.mapobj.insert(hash_str("del2"), 4962);
      // Java: this.mapobj.put("r1", 4963);
      self.mapobj.insert(hash_str("r1"), 4963);
      // Java: this.mapobj.put("r2", 4964);
      self.mapobj.insert(hash_str("r2"), 4964);
      // Java: this.mapobj.put("ca", 4965);
      self.mapobj.insert(hash_str("ca"), 4965);
      // Java: this.mapobj.put("bx", 4966);
      self.mapobj.insert(hash_str("bx"), 4966);
      // Java: this.mapobj.put("cfvo", 4967);
      self.mapobj.insert(hash_str("cfvo"), 4967);
      // Java: this.mapobj.put("minLength", 4968);
      self.mapobj.insert(hash_str("minLength"), 4968);
      // Java: this.mapobj.put("maxLength", 4969);
      self.mapobj.insert(hash_str("maxLength"), 4969);
      // Java: this.mapobj.put("showValue", 4970);
      self.mapobj.insert(hash_str("showValue"), 4970);
      // Java: this.mapobj.put("reverse", 4971);
      self.mapobj.insert(hash_str("reverse"), 4971);
      // Java: this.mapobj.put("gte", 4972);
      self.mapobj.insert(hash_str("gte"), 4972);
      // Java: this.mapobj.put("left", 4973);
      self.mapobj.insert(hash_str("left"), 4973);
      // Java: this.mapobj.put("right", 4974);
      self.mapobj.insert(hash_str("right"), 4974);
      // Java: this.mapobj.put("footer", 4975);
      self.mapobj.insert(hash_str("footer"), 4975);
      // Java: this.mapobj.put("horizontalCentered", 4976);
      self.mapobj.insert(hash_str("horizontalCentered"), 4976);
      // Java: this.mapobj.put("verticalCentered", 4977);
      self.mapobj.insert(hash_str("verticalCentered"), 4977);
      // Java: this.mapobj.put("headings", 4978);
      self.mapobj.insert(hash_str("headings"), 4978);
      // Java: this.mapobj.put("gridLines", 4979);
      self.mapobj.insert(hash_str("gridLines"), 4979);
      // Java: this.mapobj.put("gridLinesSet", 4980);
      self.mapobj.insert(hash_str("gridLinesSet"), 4980);
      // Java: this.mapobj.put("paperSize", 4981);
      self.mapobj.insert(hash_str("paperSize"), 4981);
      // Java: this.mapobj.put("paperHeight", 4982);
      self.mapobj.insert(hash_str("paperHeight"), 4982);
      // Java: this.mapobj.put("paperWidth", 4983);
      self.mapobj.insert(hash_str("paperWidth"), 4983);
      // Java: this.mapobj.put("firstPageNumber", 4984);
      self.mapobj.insert(hash_str("firstPageNumber"), 4984);
      // Java: this.mapobj.put("fitToWidth", 4985);
      self.mapobj.insert(hash_str("fitToWidth"), 4985);
      // Java: this.mapobj.put("fitToHeight", 4986);
      self.mapobj.insert(hash_str("fitToHeight"), 4986);
      // Java: this.mapobj.put("pageOrder", 4987);
      self.mapobj.insert(hash_str("pageOrder"), 4987);
      // Java: this.mapobj.put("orientation", 4988);
      self.mapobj.insert(hash_str("orientation"), 4988);
      // Java: this.mapobj.put("usePrinterDefaults", 4989);
      self.mapobj.insert(hash_str("usePrinterDefaults"), 4989);
      // Java: this.mapobj.put("blackAndWhite", 4990);
      self.mapobj.insert(hash_str("blackAndWhite"), 4990);
      // Java: this.mapobj.put("draft", 4991);
      self.mapobj.insert(hash_str("draft"), 4991);
      // Java: this.mapobj.put("cellComments", 4992);
      self.mapobj.insert(hash_str("cellComments"), 4992);
      // Java: this.mapobj.put("useFirstPageNumber", 4993);
      self.mapobj.insert(hash_str("useFirstPageNumber"), 4993);
      // Java: this.mapobj.put("errors", 4994);
      self.mapobj.insert(hash_str("errors"), 4994);
      // Java: this.mapobj.put("horizontalDpi", 4995);
      self.mapobj.insert(hash_str("horizontalDpi"), 4995);
      // Java: this.mapobj.put("verticalDpi", 4996);
      self.mapobj.insert(hash_str("verticalDpi"), 4996);
      // Java: this.mapobj.put("copies", 4997);
      self.mapobj.insert(hash_str("copies"), 4997);
      // Java: this.mapobj.put("oddHeader", 4998);
      self.mapobj.insert(hash_str("oddHeader"), 4998);
      // Java: this.mapobj.put("oddFooter", 4999);
      self.mapobj.insert(hash_str("oddFooter"), 4999);
      // Java: this.mapobj.put("evenHeader", 5000);
      self.mapobj.insert(hash_str("evenHeader"), 5000);
      // Java: this.mapobj.put("evenFooter", 5001);
      self.mapobj.insert(hash_str("evenFooter"), 5001);
      // Java: this.mapobj.put("firstHeader", 5002);
      self.mapobj.insert(hash_str("firstHeader"), 5002);
      // Java: this.mapobj.put("firstFooter", 5003);
      self.mapobj.insert(hash_str("firstFooter"), 5003);
      // Java: this.mapobj.put("differentOddEven", 5004);
      self.mapobj.insert(hash_str("differentOddEven"), 5004);
      // Java: this.mapobj.put("differentFirst", 5005);
      self.mapobj.insert(hash_str("differentFirst"), 5005);
      // Java: this.mapobj.put("scaleWithDoc", 5006);
      self.mapobj.insert(hash_str("scaleWithDoc"), 5006);
      // Java: this.mapobj.put("alignWithMargins", 5007);
      self.mapobj.insert(hash_str("alignWithMargins"), 5007);
      // Java: this.mapobj.put("scenario", 5008);
      self.mapobj.insert(hash_str("scenario"), 5008);
      // Java: this.mapobj.put("current", 5009);
      self.mapobj.insert(hash_str("current"), 5009);
      // Java: this.mapobj.put("show", 5010);
      self.mapobj.insert(hash_str("show"), 5010);
      // Java: this.mapobj.put("password", 5011);
      self.mapobj.insert(hash_str("password"), 5011);
      // Java: this.mapobj.put("algorithmName", 5012);
      self.mapobj.insert(hash_str("algorithmName"), 5012);
      // Java: this.mapobj.put("hashValue", 5013);
      self.mapobj.insert(hash_str("hashValue"), 5013);
      // Java: this.mapobj.put("saltValue", 5014);
      self.mapobj.insert(hash_str("saltValue"), 5014);
      // Java: this.mapobj.put("spinCount", 5015);
      self.mapobj.insert(hash_str("spinCount"), 5015);
      // Java: this.mapobj.put("objects", 5016);
      self.mapobj.insert(hash_str("objects"), 5016);
      // Java: this.mapobj.put("formatCells", 5017);
      self.mapobj.insert(hash_str("formatCells"), 5017);
      // Java: this.mapobj.put("formatColumns", 5018);
      self.mapobj.insert(hash_str("formatColumns"), 5018);
      // Java: this.mapobj.put("formatRows", 5019);
      self.mapobj.insert(hash_str("formatRows"), 5019);
      // Java: this.mapobj.put("insertColumns", 5020);
      self.mapobj.insert(hash_str("insertColumns"), 5020);
      // Java: this.mapobj.put("insertRows", 5021);
      self.mapobj.insert(hash_str("insertRows"), 5021);
      // Java: this.mapobj.put("insertHyperlinks", 5022);
      self.mapobj.insert(hash_str("insertHyperlinks"), 5022);
      // Java: this.mapobj.put("deleteColumns", 5023);
      self.mapobj.insert(hash_str("deleteColumns"), 5023);
      // Java: this.mapobj.put("deleteRows", 5024);
      self.mapobj.insert(hash_str("deleteRows"), 5024);
      // Java: this.mapobj.put("selectLockedCells", 5025);
      self.mapobj.insert(hash_str("selectLockedCells"), 5025);
      // Java: this.mapobj.put("sort", 5026);
      self.mapobj.insert(hash_str("sort"), 5026);
      // Java: this.mapobj.put("pivotTables", 5027);
      self.mapobj.insert(hash_str("pivotTables"), 5027);
      // Java: this.mapobj.put("selectUnlockedCells", 5028);
      self.mapobj.insert(hash_str("selectUnlockedCells"), 5028);
      // Java: this.mapobj.put("protectedRange", 5029);
      self.mapobj.insert(hash_str("protectedRange"), 5029);
      // Java: this.mapobj.put("securityDescriptor", 5030);
      self.mapobj.insert(hash_str("securityDescriptor"), 5030);
      // Java: this.mapobj.put("inputCells", 5031);
      self.mapobj.insert(hash_str("inputCells"), 5031);
      // Java: this.mapobj.put("undone", 5032);
      self.mapobj.insert(hash_str("undone"), 5032);
      // Java: this.mapobj.put("cellWatch", 5033);
      self.mapobj.insert(hash_str("cellWatch"), 5033);
      // Java: this.mapobj.put("zoomToFit", 5034);
      self.mapobj.insert(hash_str("zoomToFit"), 5034);
      // Java: this.mapobj.put("content", 5035);
      self.mapobj.insert(hash_str("content"), 5035);
      // Java: this.mapobj.put("customPr", 5036);
      self.mapobj.insert(hash_str("customPr"), 5036);
      // Java: this.mapobj.put("oleObject", 5037);
      self.mapobj.insert(hash_str("oleObject"), 5037);
      // Java: this.mapobj.put("objectPr", 5038);
      self.mapobj.insert(hash_str("objectPr"), 5038);
      // Java: this.mapobj.put("progId", 5039);
      self.mapobj.insert(hash_str("progId"), 5039);
      // Java: this.mapobj.put("dvAspect", 5040);
      self.mapobj.insert(hash_str("dvAspect"), 5040);
      // Java: this.mapobj.put("oleUpdate", 5041);
      self.mapobj.insert(hash_str("oleUpdate"), 5041);
      // Java: this.mapobj.put("autoLoad", 5042);
      self.mapobj.insert(hash_str("autoLoad"), 5042);
      // Java: this.mapobj.put("autoPict", 5043);
      self.mapobj.insert(hash_str("autoPict"), 5043);
      // Java: this.mapobj.put("macro", 5044);
      self.mapobj.insert(hash_str("macro"), 5044);
      // Java: this.mapobj.put("dde", 5045);
      self.mapobj.insert(hash_str("dde"), 5045);
      // Java: this.mapobj.put("webPublishItem", 5046);
      self.mapobj.insert(hash_str("webPublishItem"), 5046);
      // Java: this.mapobj.put("divId", 5047);
      self.mapobj.insert(hash_str("divId"), 5047);
      // Java: this.mapobj.put("sourceType", 5048);
      self.mapobj.insert(hash_str("sourceType"), 5048);
      // Java: this.mapobj.put("sourceRef", 5049);
      self.mapobj.insert(hash_str("sourceRef"), 5049);
      // Java: this.mapobj.put("sourceObject", 5050);
      self.mapobj.insert(hash_str("sourceObject"), 5050);
      // Java: this.mapobj.put("destinationFile", 5051);
      self.mapobj.insert(hash_str("destinationFile"), 5051);
      // Java: this.mapobj.put("title", 5052);
      self.mapobj.insert(hash_str("title"), 5052);
      // Java: this.mapobj.put("autoRepublish", 5053);
      self.mapobj.insert(hash_str("autoRepublish"), 5053);
      // Java: this.mapobj.put("control", 5054);
      self.mapobj.insert(hash_str("control"), 5054);
      // Java: this.mapobj.put("controlPr", 5055);
      self.mapobj.insert(hash_str("controlPr"), 5055);
      // Java: this.mapobj.put("recalcAlways", 5056);
      self.mapobj.insert(hash_str("recalcAlways"), 5056);
      // Java: this.mapobj.put("linkedCell", 5057);
      self.mapobj.insert(hash_str("linkedCell"), 5057);
      // Java: this.mapobj.put("listFillRange", 5058);
      self.mapobj.insert(hash_str("listFillRange"), 5058);
      // Java: this.mapobj.put("cf", 5059);
      self.mapobj.insert(hash_str("cf"), 5059);
      // Java: this.mapobj.put("ignoredError", 5060);
      self.mapobj.insert(hash_str("ignoredError"), 5060);
      // Java: this.mapobj.put("evalError", 5061);
      self.mapobj.insert(hash_str("evalError"), 5061);
      // Java: this.mapobj.put("twoDigitTextYear", 5062);
      self.mapobj.insert(hash_str("twoDigitTextYear"), 5062);
      // Java: this.mapobj.put("numberStoredAsText", 5063);
      self.mapobj.insert(hash_str("numberStoredAsText"), 5063);
      // Java: this.mapobj.put("formulaRange", 5064);
      self.mapobj.insert(hash_str("formulaRange"), 5064);
      // Java: this.mapobj.put("unlockedFormula", 5065);
      self.mapobj.insert(hash_str("unlockedFormula"), 5065);
      // Java: this.mapobj.put("emptyCellReference", 5066);
      self.mapobj.insert(hash_str("emptyCellReference"), 5066);
      // Java: this.mapobj.put("listDataValidation", 5067);
      self.mapobj.insert(hash_str("listDataValidation"), 5067);
      // Java: this.mapobj.put("calculatedColumn", 5068);
      self.mapobj.insert(hash_str("calculatedColumn"), 5068);
      // Java: this.mapobj.put("tablePart", 5069);
      self.mapobj.insert(hash_str("tablePart"), 5069);
      // Java: this.mapobj.put("metadata", 5070);
      self.mapobj.insert(hash_str("metadata"), 5070);
      // Java: this.mapobj.put("metadataTypes", 5071);
      self.mapobj.insert(hash_str("metadataTypes"), 5071);
      // Java: this.mapobj.put("metadataStrings", 5072);
      self.mapobj.insert(hash_str("metadataStrings"), 5072);
      // Java: this.mapobj.put("mdxMetadata", 5073);
      self.mapobj.insert(hash_str("mdxMetadata"), 5073);
      // Java: this.mapobj.put("futureMetadata", 5074);
      self.mapobj.insert(hash_str("futureMetadata"), 5074);
      // Java: this.mapobj.put("cellMetadata", 5075);
      self.mapobj.insert(hash_str("cellMetadata"), 5075);
      // Java: this.mapobj.put("valueMetadata", 5076);
      self.mapobj.insert(hash_str("valueMetadata"), 5076);
      // Java: this.mapobj.put("metadataType", 5077);
      self.mapobj.insert(hash_str("metadataType"), 5077);
      // Java: this.mapobj.put("minSupportedVersion", 5078);
      self.mapobj.insert(hash_str("minSupportedVersion"), 5078);
      // Java: this.mapobj.put("ghostRow", 5079);
      self.mapobj.insert(hash_str("ghostRow"), 5079);
      // Java: this.mapobj.put("ghostCol", 5080);
      self.mapobj.insert(hash_str("ghostCol"), 5080);
      // Java: this.mapobj.put("edit", 5081);
      self.mapobj.insert(hash_str("edit"), 5081);
      // Java: this.mapobj.put("delete", 5082);
      self.mapobj.insert(hash_str("delete"), 5082);
      // Java: this.mapobj.put("copy", 5083);
      self.mapobj.insert(hash_str("copy"), 5083);
      // Java: this.mapobj.put("pasteAll", 5084);
      self.mapobj.insert(hash_str("pasteAll"), 5084);
      // Java: this.mapobj.put("pasteFormulas", 5085);
      self.mapobj.insert(hash_str("pasteFormulas"), 5085);
      // Java: this.mapobj.put("pasteValues", 5086);
      self.mapobj.insert(hash_str("pasteValues"), 5086);
      // Java: this.mapobj.put("pasteFormats", 5087);
      self.mapobj.insert(hash_str("pasteFormats"), 5087);
      // Java: this.mapobj.put("pasteComments", 5088);
      self.mapobj.insert(hash_str("pasteComments"), 5088);
      // Java: this.mapobj.put("pasteDataValidation", 5089);
      self.mapobj.insert(hash_str("pasteDataValidation"), 5089);
      // Java: this.mapobj.put("pasteBorders", 5090);
      self.mapobj.insert(hash_str("pasteBorders"), 5090);
      // Java: this.mapobj.put("pasteColWidths", 5091);
      self.mapobj.insert(hash_str("pasteColWidths"), 5091);
      // Java: this.mapobj.put("pasteNumberFormats", 5092);
      self.mapobj.insert(hash_str("pasteNumberFormats"), 5092);
      // Java: this.mapobj.put("merge", 5093);
      self.mapobj.insert(hash_str("merge"), 5093);
      // Java: this.mapobj.put("splitFirst", 5094);
      self.mapobj.insert(hash_str("splitFirst"), 5094);
      // Java: this.mapobj.put("splitAll", 5095);
      self.mapobj.insert(hash_str("splitAll"), 5095);
      // Java: this.mapobj.put("rowColShift", 5096);
      self.mapobj.insert(hash_str("rowColShift"), 5096);
      // Java: this.mapobj.put("clearAll", 5097);
      self.mapobj.insert(hash_str("clearAll"), 5097);
      // Java: this.mapobj.put("clearFormats", 5098);
      self.mapobj.insert(hash_str("clearFormats"), 5098);
      // Java: this.mapobj.put("clearContents", 5099);
      self.mapobj.insert(hash_str("clearContents"), 5099);
      // Java: this.mapobj.put("clearComments", 5100);
      self.mapobj.insert(hash_str("clearComments"), 5100);
      // Java: this.mapobj.put("assign", 5101);
      self.mapobj.insert(hash_str("assign"), 5101);
      // Java: this.mapobj.put("coerce", 5102);
      self.mapobj.insert(hash_str("coerce"), 5102);
      // Java: this.mapobj.put("adjust", 5103);
      self.mapobj.insert(hash_str("adjust"), 5103);
      // Java: this.mapobj.put("cellMeta", 5104);
      self.mapobj.insert(hash_str("cellMeta"), 5104);
      // Java: this.mapobj.put("bk", 5105);
      self.mapobj.insert(hash_str("bk"), 5105);
      // Java: this.mapobj.put("rc", 5106);
      self.mapobj.insert(hash_str("rc"), 5106);
      // Java: this.mapobj.put("ms", 5107);
      self.mapobj.insert(hash_str("ms"), 5107);
      // Java: this.mapobj.put("p", 5108);
      self.mapobj.insert(hash_str("p"), 5108);
      // Java: this.mapobj.put("k", 5109);
      self.mapobj.insert(hash_str("k"), 5109);
      // Java: this.mapobj.put("ct", 5110);
      self.mapobj.insert(hash_str("ct"), 5110);
      // Java: this.mapobj.put("fi", 5111);
      self.mapobj.insert(hash_str("fi"), 5111);
      // Java: this.mapobj.put("ns", 5112);
      self.mapobj.insert(hash_str("ns"), 5112);
      // Java: this.mapobj.put("o", 5113);
      self.mapobj.insert(hash_str("o"), 5113);
      // Java: this.mapobj.put("np", 5114);
      self.mapobj.insert(hash_str("np"), 5114);
      // Java: this.mapobj.put("singleXmlCells", 5115);
      self.mapobj.insert(hash_str("singleXmlCells"), 5115);
      // Java: this.mapobj.put("singleXmlCell", 5116);
      self.mapobj.insert(hash_str("singleXmlCell"), 5116);
      // Java: this.mapobj.put("xmlCellPr", 5117);
      self.mapobj.insert(hash_str("xmlCellPr"), 5117);
      // Java: this.mapobj.put("xmlPr", 5118);
      self.mapobj.insert(hash_str("xmlPr"), 5118);
      // Java: this.mapobj.put("mapId", 5119);
      self.mapobj.insert(hash_str("mapId"), 5119);
      // Java: this.mapobj.put("xpath", 5120);
      self.mapobj.insert(hash_str("xpath"), 5120);
      // Java: this.mapobj.put("xmlDataType", 5121);
      self.mapobj.insert(hash_str("xmlDataType"), 5121);
      // Java: this.mapobj.put("styleSheet", 5122);
      self.mapobj.insert(hash_str("styleSheet"), 5122);
      // Java: this.mapobj.put("numFmts", 5123);
      self.mapobj.insert(hash_str("numFmts"), 5123);
      // Java: this.mapobj.put("fonts", 5124);
      self.mapobj.insert(hash_str("fonts"), 5124);
      // Java: this.mapobj.put("fills", 5125);
      self.mapobj.insert(hash_str("fills"), 5125);
      // Java: this.mapobj.put("borders", 5126);
      self.mapobj.insert(hash_str("borders"), 5126);
      // Java: this.mapobj.put("cellStyleXfs", 5127);
      self.mapobj.insert(hash_str("cellStyleXfs"), 5127);
      // Java: this.mapobj.put("cellXfs", 5128);
      self.mapobj.insert(hash_str("cellXfs"), 5128);
      // Java: this.mapobj.put("cellStyles", 5129);
      self.mapobj.insert(hash_str("cellStyles"), 5129);
      // Java: this.mapobj.put("dxfs", 5130);
      self.mapobj.insert(hash_str("dxfs"), 5130);
      // Java: this.mapobj.put("tableStyles", 5131);
      self.mapobj.insert(hash_str("tableStyles"), 5131);
      // Java: this.mapobj.put("colors", 5132);
      self.mapobj.insert(hash_str("colors"), 5132);
      // Java: this.mapobj.put("horizontal", 5133);
      self.mapobj.insert(hash_str("horizontal"), 5133);
      // Java: this.mapobj.put("vertical", 5134);
      self.mapobj.insert(hash_str("vertical"), 5134);
      // Java: this.mapobj.put("textRotation", 5135);
      self.mapobj.insert(hash_str("textRotation"), 5135);
      // Java: this.mapobj.put("wrapText", 5136);
      self.mapobj.insert(hash_str("wrapText"), 5136);
      // Java: this.mapobj.put("relativeIndent", 5137);
      self.mapobj.insert(hash_str("relativeIndent"), 5137);
      // Java: this.mapobj.put("justifyLastLine", 5138);
      self.mapobj.insert(hash_str("justifyLastLine"), 5138);
      // Java: this.mapobj.put("shrinkToFit", 5139);
      self.mapobj.insert(hash_str("shrinkToFit"), 5139);
      // Java: this.mapobj.put("readingOrder", 5140);
      self.mapobj.insert(hash_str("readingOrder"), 5140);
      // Java: this.mapobj.put("border", 5141);
      self.mapobj.insert(hash_str("border"), 5141);
      // Java: this.mapobj.put("end", 5142);
      self.mapobj.insert(hash_str("end"), 5142);
      // Java: this.mapobj.put("diagonal", 5143);
      self.mapobj.insert(hash_str("diagonal"), 5143);
      // Java: this.mapobj.put("diagonalUp", 5144);
      self.mapobj.insert(hash_str("diagonalUp"), 5144);
      // Java: this.mapobj.put("diagonalDown", 5145);
      self.mapobj.insert(hash_str("diagonalDown"), 5145);
      // Java: this.mapobj.put("font", 5146);
      self.mapobj.insert(hash_str("font"), 5146);
      // Java: this.mapobj.put("fill", 5147);
      self.mapobj.insert(hash_str("fill"), 5147);
      // Java: this.mapobj.put("patternFill", 5148);
      self.mapobj.insert(hash_str("patternFill"), 5148);
      // Java: this.mapobj.put("gradientFill", 5149);
      self.mapobj.insert(hash_str("gradientFill"), 5149);
      // Java: this.mapobj.put("fgColor", 5150);
      self.mapobj.insert(hash_str("fgColor"), 5150);
      // Java: this.mapobj.put("bgColor", 5151);
      self.mapobj.insert(hash_str("bgColor"), 5151);
      // Java: this.mapobj.put("patternType", 5152);
      self.mapobj.insert(hash_str("patternType"), 5152);
      // Java: this.mapobj.put("auto", 5153);
      self.mapobj.insert(hash_str("auto"), 5153);
      // Java: this.mapobj.put("indexed", 5154);
      self.mapobj.insert(hash_str("indexed"), 5154);
      // Java: this.mapobj.put("rgb", 5155);
      self.mapobj.insert(hash_str("rgb"), 5155);
      // Java: this.mapobj.put("theme", 5156);
      self.mapobj.insert(hash_str("theme"), 5156);
      // Java: this.mapobj.put("tint", 5157);
      self.mapobj.insert(hash_str("tint"), 5157);
      // Java: this.mapobj.put("stop", 5158);
      self.mapobj.insert(hash_str("stop"), 5158);
      // Java: this.mapobj.put("degree", 5159);
      self.mapobj.insert(hash_str("degree"), 5159);
      // Java: this.mapobj.put("numFmt", 5160);
      self.mapobj.insert(hash_str("numFmt"), 5160);
      // Java: this.mapobj.put("formatCode", 5161);
      self.mapobj.insert(hash_str("formatCode"), 5161);
      // Java: this.mapobj.put("xf", 5162);
      self.mapobj.insert(hash_str("xf"), 5162);
      // Java: this.mapobj.put("protection", 5163);
      self.mapobj.insert(hash_str("protection"), 5163);
      // Java: this.mapobj.put("fillId", 5164);
      self.mapobj.insert(hash_str("fillId"), 5164);
      // Java: this.mapobj.put("borderId", 5165);
      self.mapobj.insert(hash_str("borderId"), 5165);
      // Java: this.mapobj.put("xfId", 5166);
      self.mapobj.insert(hash_str("xfId"), 5166);
      // Java: this.mapobj.put("pivotButton", 5167);
      self.mapobj.insert(hash_str("pivotButton"), 5167);
      // Java: this.mapobj.put("applyNumberFormat", 5168);
      self.mapobj.insert(hash_str("applyNumberFormat"), 5168);
      // Java: this.mapobj.put("applyFont", 5169);
      self.mapobj.insert(hash_str("applyFont"), 5169);
      // Java: this.mapobj.put("applyFill", 5170);
      self.mapobj.insert(hash_str("applyFill"), 5170);
      // Java: this.mapobj.put("applyBorder", 5171);
      self.mapobj.insert(hash_str("applyBorder"), 5171);
      // Java: this.mapobj.put("applyAlignment", 5172);
      self.mapobj.insert(hash_str("applyAlignment"), 5172);
      // Java: this.mapobj.put("applyProtection", 5173);
      self.mapobj.insert(hash_str("applyProtection"), 5173);
      // Java: this.mapobj.put("cellStyle", 5174);
      self.mapobj.insert(hash_str("cellStyle"), 5174);
      // Java: this.mapobj.put("builtinId", 5175);
      self.mapobj.insert(hash_str("builtinId"), 5175);
      // Java: this.mapobj.put("iLevel", 5176);
      self.mapobj.insert(hash_str("iLevel"), 5176);
      // Java: this.mapobj.put("customBuiltin", 5177);
      self.mapobj.insert(hash_str("customBuiltin"), 5177);
      // Java: this.mapobj.put("indexedColors", 5178);
      self.mapobj.insert(hash_str("indexedColors"), 5178);
      // Java: this.mapobj.put("mruColors", 5179);
      self.mapobj.insert(hash_str("mruColors"), 5179);
      // Java: this.mapobj.put("rgbColor", 5180);
      self.mapobj.insert(hash_str("rgbColor"), 5180);
      // Java: this.mapobj.put("tableStyle", 5181);
      self.mapobj.insert(hash_str("tableStyle"), 5181);
      // Java: this.mapobj.put("defaultTableStyle", 5182);
      self.mapobj.insert(hash_str("defaultTableStyle"), 5182);
      // Java: this.mapobj.put("defaultPivotStyle", 5183);
      self.mapobj.insert(hash_str("defaultPivotStyle"), 5183);
      // Java: this.mapobj.put("tableStyleElement", 5184);
      self.mapobj.insert(hash_str("tableStyleElement"), 5184);
      // Java: this.mapobj.put("table", 5185);
      self.mapobj.insert(hash_str("table"), 5185);
      // Java: this.mapobj.put("size", 5186);
      self.mapobj.insert(hash_str("size"), 5186);
      // Java: this.mapobj.put("autoFormatId", 5187);
      self.mapobj.insert(hash_str("autoFormatId"), 5187);
      // Java: this.mapobj.put("applyNumberFormats", 5188);
      self.mapobj.insert(hash_str("applyNumberFormats"), 5188);
      // Java: this.mapobj.put("applyBorderFormats", 5189);
      self.mapobj.insert(hash_str("applyBorderFormats"), 5189);
      // Java: this.mapobj.put("applyFontFormats", 5190);
      self.mapobj.insert(hash_str("applyFontFormats"), 5190);
      // Java: this.mapobj.put("applyPatternFormats", 5191);
      self.mapobj.insert(hash_str("applyPatternFormats"), 5191);
      // Java: this.mapobj.put("applyAlignmentFormats", 5192);
      self.mapobj.insert(hash_str("applyAlignmentFormats"), 5192);
      // Java: this.mapobj.put("applyWidthHeightFormats", 5193);
      self.mapobj.insert(hash_str("applyWidthHeightFormats"), 5193);
      // Java: this.mapobj.put("externalLink", 5194);
      self.mapobj.insert(hash_str("externalLink"), 5194);
      // Java: this.mapobj.put("externalBook", 5195);
      self.mapobj.insert(hash_str("externalBook"), 5195);
      // Java: this.mapobj.put("ddeLink", 5196);
      self.mapobj.insert(hash_str("ddeLink"), 5196);
      // Java: this.mapobj.put("oleLink", 5197);
      self.mapobj.insert(hash_str("oleLink"), 5197);
      // Java: this.mapobj.put("sheetNames", 5198);
      self.mapobj.insert(hash_str("sheetNames"), 5198);
      // Java: this.mapobj.put("definedNames", 5199);
      self.mapobj.insert(hash_str("definedNames"), 5199);
      // Java: this.mapobj.put("sheetDataSet", 5200);
      self.mapobj.insert(hash_str("sheetDataSet"), 5200);
      // Java: this.mapobj.put("sheetName", 5201);
      self.mapobj.insert(hash_str("sheetName"), 5201);
      // Java: this.mapobj.put("definedName", 5202);
      self.mapobj.insert(hash_str("definedName"), 5202);
      // Java: this.mapobj.put("refersTo", 5203);
      self.mapobj.insert(hash_str("refersTo"), 5203);
      // Java: this.mapobj.put("refreshError", 5204);
      self.mapobj.insert(hash_str("refreshError"), 5204);
      // Java: this.mapobj.put("ddeItems", 5205);
      self.mapobj.insert(hash_str("ddeItems"), 5205);
      // Java: this.mapobj.put("ddeService", 5206);
      self.mapobj.insert(hash_str("ddeService"), 5206);
      // Java: this.mapobj.put("ddeTopic", 5207);
      self.mapobj.insert(hash_str("ddeTopic"), 5207);
      // Java: this.mapobj.put("ddeItem", 5208);
      self.mapobj.insert(hash_str("ddeItem"), 5208);
      // Java: this.mapobj.put("values", 5209);
      self.mapobj.insert(hash_str("values"), 5209);
      // Java: this.mapobj.put("ole", 5210);
      self.mapobj.insert(hash_str("ole"), 5210);
      // Java: this.mapobj.put("advise", 5211);
      self.mapobj.insert(hash_str("advise"), 5211);
      // Java: this.mapobj.put("preferPic", 5212);
      self.mapobj.insert(hash_str("preferPic"), 5212);
      // Java: this.mapobj.put("rows", 5213);
      self.mapobj.insert(hash_str("rows"), 5213);
      // Java: this.mapobj.put("oleItems", 5214);
      self.mapobj.insert(hash_str("oleItems"), 5214);
      // Java: this.mapobj.put("oleItem", 5215);
      self.mapobj.insert(hash_str("oleItem"), 5215);
      // Java: this.mapobj.put("icon", 5216);
      self.mapobj.insert(hash_str("icon"), 5216);
      // Java: this.mapobj.put("tableColumns", 5217);
      self.mapobj.insert(hash_str("tableColumns"), 5217);
      // Java: this.mapobj.put("tableStyleInfo", 5218);
      self.mapobj.insert(hash_str("tableStyleInfo"), 5218);
      // Java: this.mapobj.put("displayName", 5219);
      self.mapobj.insert(hash_str("displayName"), 5219);
      // Java: this.mapobj.put("tableType", 5220);
      self.mapobj.insert(hash_str("tableType"), 5220);
      // Java: this.mapobj.put("headerRowCount", 5221);
      self.mapobj.insert(hash_str("headerRowCount"), 5221);
      // Java: this.mapobj.put("insertRow", 5222);
      self.mapobj.insert(hash_str("insertRow"), 5222);
      // Java: this.mapobj.put("insertRowShift", 5223);
      self.mapobj.insert(hash_str("insertRowShift"), 5223);
      // Java: this.mapobj.put("totalsRowCount", 5224);
      self.mapobj.insert(hash_str("totalsRowCount"), 5224);
      // Java: this.mapobj.put("totalsRowShown", 5225);
      self.mapobj.insert(hash_str("totalsRowShown"), 5225);
      // Java: this.mapobj.put("headerRowDxfId", 5226);
      self.mapobj.insert(hash_str("headerRowDxfId"), 5226);
      // Java: this.mapobj.put("dataDxfId", 5227);
      self.mapobj.insert(hash_str("dataDxfId"), 5227);
      // Java: this.mapobj.put("totalsRowDxfId", 5228);
      self.mapobj.insert(hash_str("totalsRowDxfId"), 5228);
      // Java: this.mapobj.put("headerRowBorderDxfId", 5229);
      self.mapobj.insert(hash_str("headerRowBorderDxfId"), 5229);
      // Java: this.mapobj.put("tableBorderDxfId", 5230);
      self.mapobj.insert(hash_str("tableBorderDxfId"), 5230);
      // Java: this.mapobj.put("totalsRowBorderDxfId", 5231);
      self.mapobj.insert(hash_str("totalsRowBorderDxfId"), 5231);
      // Java: this.mapobj.put("headerRowCellStyle", 5232);
      self.mapobj.insert(hash_str("headerRowCellStyle"), 5232);
      // Java: this.mapobj.put("dataCellStyle", 5233);
      self.mapobj.insert(hash_str("dataCellStyle"), 5233);
      // Java: this.mapobj.put("totalsRowCellStyle", 5234);
      self.mapobj.insert(hash_str("totalsRowCellStyle"), 5234);
      // Java: this.mapobj.put("showFirstColumn", 5235);
      self.mapobj.insert(hash_str("showFirstColumn"), 5235);
      // Java: this.mapobj.put("showColumnStripes", 5236);
      self.mapobj.insert(hash_str("showColumnStripes"), 5236);
      // Java: this.mapobj.put("tableColumn", 5237);
      self.mapobj.insert(hash_str("tableColumn"), 5237);
      // Java: this.mapobj.put("calculatedColumnFormula", 5238);
      self.mapobj.insert(hash_str("calculatedColumnFormula"), 5238);
      // Java: this.mapobj.put("totalsRowFormula", 5239);
      self.mapobj.insert(hash_str("totalsRowFormula"), 5239);
      // Java: this.mapobj.put("xmlColumnPr", 5240);
      self.mapobj.insert(hash_str("xmlColumnPr"), 5240);
      // Java: this.mapobj.put("totalsRowFunction", 5241);
      self.mapobj.insert(hash_str("totalsRowFunction"), 5241);
      // Java: this.mapobj.put("totalsRowLabel", 5242);
      self.mapobj.insert(hash_str("totalsRowLabel"), 5242);
      // Java: this.mapobj.put("queryTableFieldId", 5243);
      self.mapobj.insert(hash_str("queryTableFieldId"), 5243);
      // Java: this.mapobj.put("denormalized", 5244);
      self.mapobj.insert(hash_str("denormalized"), 5244);
      // Java: this.mapobj.put("volTypes", 5245);
      self.mapobj.insert(hash_str("volTypes"), 5245);
      // Java: this.mapobj.put("volType", 5246);
      self.mapobj.insert(hash_str("volType"), 5246);
      // Java: this.mapobj.put("main", 5247);
      self.mapobj.insert(hash_str("main"), 5247);
      // Java: this.mapobj.put("tp", 5248);
      self.mapobj.insert(hash_str("tp"), 5248);
      // Java: this.mapobj.put("first", 5249);
      self.mapobj.insert(hash_str("first"), 5249);
      // Java: this.mapobj.put("stp", 5250);
      self.mapobj.insert(hash_str("stp"), 5250);
      // Java: this.mapobj.put("tr", 5251);
      self.mapobj.insert(hash_str("tr"), 5251);
      // Java: this.mapobj.put("workbook", 5252);
      self.mapobj.insert(hash_str("workbook"), 5252);
      // Java: this.mapobj.put("fileVersion", 5253);
      self.mapobj.insert(hash_str("fileVersion"), 5253);
      // Java: this.mapobj.put("fileSharing", 5254);
      self.mapobj.insert(hash_str("fileSharing"), 5254);
      // Java: this.mapobj.put("workbookPr", 5255);
      self.mapobj.insert(hash_str("workbookPr"), 5255);
      // Java: this.mapobj.put("workbookProtection", 5256);
      self.mapobj.insert(hash_str("workbookProtection"), 5256);
      // Java: this.mapobj.put("bookViews", 5257);
      self.mapobj.insert(hash_str("bookViews"), 5257);
      // Java: this.mapobj.put("sheets", 5258);
      self.mapobj.insert(hash_str("sheets"), 5258);
      // Java: this.mapobj.put("functionGroups", 5259);
      self.mapobj.insert(hash_str("functionGroups"), 5259);
      // Java: this.mapobj.put("externalReferences", 5260);
      self.mapobj.insert(hash_str("externalReferences"), 5260);
      // Java: this.mapobj.put("calcPr", 5261);
      self.mapobj.insert(hash_str("calcPr"), 5261);
      // Java: this.mapobj.put("oleSize", 5262);
      self.mapobj.insert(hash_str("oleSize"), 5262);
      // Java: this.mapobj.put("customWorkbookViews", 5263);
      self.mapobj.insert(hash_str("customWorkbookViews"), 5263);
      // Java: this.mapobj.put("pivotCaches", 5264);
      self.mapobj.insert(hash_str("pivotCaches"), 5264);
      // Java: this.mapobj.put("smartTagPr", 5265);
      self.mapobj.insert(hash_str("smartTagPr"), 5265);
      // Java: this.mapobj.put("smartTagTypes", 5266);
      self.mapobj.insert(hash_str("smartTagTypes"), 5266);
      // Java: this.mapobj.put("webPublishing", 5267);
      self.mapobj.insert(hash_str("webPublishing"), 5267);
      // Java: this.mapobj.put("fileRecoveryPr", 5268);
      self.mapobj.insert(hash_str("fileRecoveryPr"), 5268);
      // Java: this.mapobj.put("webPublishObjects", 5269);
      self.mapobj.insert(hash_str("webPublishObjects"), 5269);
      // Java: this.mapobj.put("conformance", 5270);
      self.mapobj.insert(hash_str("conformance"), 5270);
      // Java: this.mapobj.put("appName", 5271);
      self.mapobj.insert(hash_str("appName"), 5271);
      // Java: this.mapobj.put("lastEdited", 5272);
      self.mapobj.insert(hash_str("lastEdited"), 5272);
      // Java: this.mapobj.put("lowestEdited", 5273);
      self.mapobj.insert(hash_str("lowestEdited"), 5273);
      // Java: this.mapobj.put("rupBuild", 5274);
      self.mapobj.insert(hash_str("rupBuild"), 5274);
      // Java: this.mapobj.put("workbookView", 5275);
      self.mapobj.insert(hash_str("workbookView"), 5275);
      // Java: this.mapobj.put("visibility", 5276);
      self.mapobj.insert(hash_str("visibility"), 5276);
      // Java: this.mapobj.put("minimized", 5277);
      self.mapobj.insert(hash_str("minimized"), 5277);
      // Java: this.mapobj.put("showHorizontalScroll", 5278);
      self.mapobj.insert(hash_str("showHorizontalScroll"), 5278);
      // Java: this.mapobj.put("showVerticalScroll", 5279);
      self.mapobj.insert(hash_str("showVerticalScroll"), 5279);
      // Java: this.mapobj.put("showSheetTabs", 5280);
      self.mapobj.insert(hash_str("showSheetTabs"), 5280);
      // Java: this.mapobj.put("windowWidth", 5281);
      self.mapobj.insert(hash_str("windowWidth"), 5281);
      // Java: this.mapobj.put("windowHeight", 5282);
      self.mapobj.insert(hash_str("windowHeight"), 5282);
      // Java: this.mapobj.put("tabRatio", 5283);
      self.mapobj.insert(hash_str("tabRatio"), 5283);
      // Java: this.mapobj.put("firstSheet", 5284);
      self.mapobj.insert(hash_str("firstSheet"), 5284);
      // Java: this.mapobj.put("activeTab", 5285);
      self.mapobj.insert(hash_str("activeTab"), 5285);
      // Java: this.mapobj.put("autoFilterDateGrouping", 5286);
      self.mapobj.insert(hash_str("autoFilterDateGrouping"), 5286);
      // Java: this.mapobj.put("customWorkbookView", 5287);
      self.mapobj.insert(hash_str("customWorkbookView"), 5287);
      // Java: this.mapobj.put("autoUpdate", 5288);
      self.mapobj.insert(hash_str("autoUpdate"), 5288);
      // Java: this.mapobj.put("mergeInterval", 5289);
      self.mapobj.insert(hash_str("mergeInterval"), 5289);
      // Java: this.mapobj.put("changesSavedWin", 5290);
      self.mapobj.insert(hash_str("changesSavedWin"), 5290);
      // Java: this.mapobj.put("onlySync", 5291);
      self.mapobj.insert(hash_str("onlySync"), 5291);
      // Java: this.mapobj.put("personalView", 5292);
      self.mapobj.insert(hash_str("personalView"), 5292);
      // Java: this.mapobj.put("includePrintSettings", 5293);
      self.mapobj.insert(hash_str("includePrintSettings"), 5293);
      // Java: this.mapobj.put("includeHiddenRowCol", 5294);
      self.mapobj.insert(hash_str("includeHiddenRowCol"), 5294);
      // Java: this.mapobj.put("maximized", 5295);
      self.mapobj.insert(hash_str("maximized"), 5295);
      // Java: this.mapobj.put("activeSheetId", 5296);
      self.mapobj.insert(hash_str("activeSheetId"), 5296);
      // Java: this.mapobj.put("showFormulaBar", 5297);
      self.mapobj.insert(hash_str("showFormulaBar"), 5297);
      // Java: this.mapobj.put("showStatusbar", 5298);
      self.mapobj.insert(hash_str("showStatusbar"), 5298);
      // Java: this.mapobj.put("showComments", 5299);
      self.mapobj.insert(hash_str("showComments"), 5299);
      // Java: this.mapobj.put("showObjects", 5300);
      self.mapobj.insert(hash_str("showObjects"), 5300);
      // Java: this.mapobj.put("date1904", 5301);
      self.mapobj.insert(hash_str("date1904"), 5301);
      // Java: this.mapobj.put("dateCompatibility", 5302);
      self.mapobj.insert(hash_str("dateCompatibility"), 5302);
      // Java: this.mapobj.put("showBorderUnselectedTables", 5303);
      self.mapobj.insert(hash_str("showBorderUnselectedTables"), 5303);
      // Java: this.mapobj.put("filterPrivacy", 5304);
      self.mapobj.insert(hash_str("filterPrivacy"), 5304);
      // Java: this.mapobj.put("promptedSolutions", 5305);
      self.mapobj.insert(hash_str("promptedSolutions"), 5305);
      // Java: this.mapobj.put("showInkAnnotation", 5306);
      self.mapobj.insert(hash_str("showInkAnnotation"), 5306);
      // Java: this.mapobj.put("backupFile", 5307);
      self.mapobj.insert(hash_str("backupFile"), 5307);
      // Java: this.mapobj.put("saveExternalLinkValues", 5308);
      self.mapobj.insert(hash_str("saveExternalLinkValues"), 5308);
      // Java: this.mapobj.put("updateLinks", 5309);
      self.mapobj.insert(hash_str("updateLinks"), 5309);
      // Java: this.mapobj.put("hidePivotFieldList", 5310);
      self.mapobj.insert(hash_str("hidePivotFieldList"), 5310);
      // Java: this.mapobj.put("showPivotChartFilter", 5311);
      self.mapobj.insert(hash_str("showPivotChartFilter"), 5311);
      // Java: this.mapobj.put("allowRefreshQuery", 5312);
      self.mapobj.insert(hash_str("allowRefreshQuery"), 5312);
      // Java: this.mapobj.put("publishItems", 5313);
      self.mapobj.insert(hash_str("publishItems"), 5313);
      // Java: this.mapobj.put("checkCompatibility", 5314);
      self.mapobj.insert(hash_str("checkCompatibility"), 5314);
      // Java: this.mapobj.put("autoCompressPictures", 5315);
      self.mapobj.insert(hash_str("autoCompressPictures"), 5315);
      // Java: this.mapobj.put("refreshAllConnections", 5316);
      self.mapobj.insert(hash_str("refreshAllConnections"), 5316);
      // Java: this.mapobj.put("defaultThemeVersion", 5317);
      self.mapobj.insert(hash_str("defaultThemeVersion"), 5317);
      // Java: this.mapobj.put("embed", 5318);
      self.mapobj.insert(hash_str("embed"), 5318);
      // Java: this.mapobj.put("smartTagType", 5319);
      self.mapobj.insert(hash_str("smartTagType"), 5319);
      // Java: this.mapobj.put("namespaceUri", 5320);
      self.mapobj.insert(hash_str("namespaceUri"), 5320);
      // Java: this.mapobj.put("autoRecover", 5321);
      self.mapobj.insert(hash_str("autoRecover"), 5321);
      // Java: this.mapobj.put("crashSave", 5322);
      self.mapobj.insert(hash_str("crashSave"), 5322);
      // Java: this.mapobj.put("dataExtractLoad", 5323);
      self.mapobj.insert(hash_str("dataExtractLoad"), 5323);
      // Java: this.mapobj.put("repairLoad", 5324);
      self.mapobj.insert(hash_str("repairLoad"), 5324);
      // Java: this.mapobj.put("calcId", 5325);
      self.mapobj.insert(hash_str("calcId"), 5325);
      // Java: this.mapobj.put("calcMode", 5326);
      self.mapobj.insert(hash_str("calcMode"), 5326);
      // Java: this.mapobj.put("refMode", 5327);
      self.mapobj.insert(hash_str("refMode"), 5327);
      // Java: this.mapobj.put("iterate", 5328);
      self.mapobj.insert(hash_str("iterate"), 5328);
      // Java: this.mapobj.put("iterateCount", 5329);
      self.mapobj.insert(hash_str("iterateCount"), 5329);
      // Java: this.mapobj.put("iterateDelta", 5330);
      self.mapobj.insert(hash_str("iterateDelta"), 5330);
      // Java: this.mapobj.put("fullPrecision", 5331);
      self.mapobj.insert(hash_str("fullPrecision"), 5331);
      // Java: this.mapobj.put("calcCompleted", 5332);
      self.mapobj.insert(hash_str("calcCompleted"), 5332);
      // Java: this.mapobj.put("calcOnSave", 5333);
      self.mapobj.insert(hash_str("calcOnSave"), 5333);
      // Java: this.mapobj.put("concurrentCalc", 5334);
      self.mapobj.insert(hash_str("concurrentCalc"), 5334);
      // Java: this.mapobj.put("concurrentManualCount", 5335);
      self.mapobj.insert(hash_str("concurrentManualCount"), 5335);
      // Java: this.mapobj.put("forceFullCalc", 5336);
      self.mapobj.insert(hash_str("forceFullCalc"), 5336);
      // Java: this.mapobj.put("vbProcedure", 5337);
      self.mapobj.insert(hash_str("vbProcedure"), 5337);
      // Java: this.mapobj.put("xlm", 5338);
      self.mapobj.insert(hash_str("xlm"), 5338);
      // Java: this.mapobj.put("publishToServer", 5339);
      self.mapobj.insert(hash_str("publishToServer"), 5339);
      // Java: this.mapobj.put("workbookParameter", 5340);
      self.mapobj.insert(hash_str("workbookParameter"), 5340);
      // Java: this.mapobj.put("externalReference", 5341);
      self.mapobj.insert(hash_str("externalReference"), 5341);
      // Java: this.mapobj.put("pivotCache", 5342);
      self.mapobj.insert(hash_str("pivotCache"), 5342);
      // Java: this.mapobj.put("readOnlyRecommended", 5343);
      self.mapobj.insert(hash_str("readOnlyRecommended"), 5343);
      // Java: this.mapobj.put("reservationPassword", 5344);
      self.mapobj.insert(hash_str("reservationPassword"), 5344);
      // Java: this.mapobj.put("workbookPassword", 5345);
      self.mapobj.insert(hash_str("workbookPassword"), 5345);
      // Java: this.mapobj.put("workbookPasswordCharacterSet", 5346);
      self.mapobj.insert(hash_str("workbookPasswordCharacterSet"), 5346);
      // Java: this.mapobj.put("revisionsPassword", 5347);
      self.mapobj.insert(hash_str("revisionsPassword"), 5347);
      // Java: this.mapobj.put("revisionsPasswordCharacterSet", 5348);
      self.mapobj.insert(hash_str("revisionsPasswordCharacterSet"), 5348);
      // Java: this.mapobj.put("lockStructure", 5349);
      self.mapobj.insert(hash_str("lockStructure"), 5349);
      // Java: this.mapobj.put("lockWindows", 5350);
      self.mapobj.insert(hash_str("lockWindows"), 5350);
      // Java: this.mapobj.put("lockRevision", 5351);
      self.mapobj.insert(hash_str("lockRevision"), 5351);
      // Java: this.mapobj.put("revisionsAlgorithmName", 5352);
      self.mapobj.insert(hash_str("revisionsAlgorithmName"), 5352);
      // Java: this.mapobj.put("revisionsHashValue", 5353);
      self.mapobj.insert(hash_str("revisionsHashValue"), 5353);
      // Java: this.mapobj.put("revisionsSaltValue", 5354);
      self.mapobj.insert(hash_str("revisionsSaltValue"), 5354);
      // Java: this.mapobj.put("revisionsSpinCount", 5355);
      self.mapobj.insert(hash_str("revisionsSpinCount"), 5355);
      // Java: this.mapobj.put("workbookAlgorithmName", 5356);
      self.mapobj.insert(hash_str("workbookAlgorithmName"), 5356);
      // Java: this.mapobj.put("workbookHashValue", 5357);
      self.mapobj.insert(hash_str("workbookHashValue"), 5357);
      // Java: this.mapobj.put("workbookSaltValue", 5358);
      self.mapobj.insert(hash_str("workbookSaltValue"), 5358);
      // Java: this.mapobj.put("workbookSpinCount", 5359);
      self.mapobj.insert(hash_str("workbookSpinCount"), 5359);
      // Java: this.mapobj.put("css", 5360);
      self.mapobj.insert(hash_str("css"), 5360);
      // Java: this.mapobj.put("thicket", 5361);
      self.mapobj.insert(hash_str("thicket"), 5361);
      // Java: this.mapobj.put("longFileNames", 5362);
      self.mapobj.insert(hash_str("longFileNames"), 5362);
      // Java: this.mapobj.put("vml", 5363);
      self.mapobj.insert(hash_str("vml"), 5363);
      // Java: this.mapobj.put("allowPng", 5364);
      self.mapobj.insert(hash_str("allowPng"), 5364);
      // Java: this.mapobj.put("targetScreenSize", 5365);
      self.mapobj.insert(hash_str("targetScreenSize"), 5365);
      // Java: this.mapobj.put("dpi", 5366);
      self.mapobj.insert(hash_str("dpi"), 5366);
      // Java: this.mapobj.put("functionGroup", 5367);
      self.mapobj.insert(hash_str("functionGroup"), 5367);
      // Java: this.mapobj.put("builtInGroupCount", 5368);
      self.mapobj.insert(hash_str("builtInGroupCount"), 5368);
      // Java: this.mapobj.put("webPublishObject", 5369);
      self.mapobj.insert(hash_str("webPublishObject"), 5369);
      // Java: this.mapobj.put("sparklineGroup", 5370);
      self.mapobj.insert(hash_str("sparklineGroup"), 5370);
      // Java: this.mapobj.put("sparklineGroups", 5371);
      self.mapobj.insert(hash_str("sparklineGroups"), 5371);
      // Java: this.mapobj.put("colorSeries", 5372);
      self.mapobj.insert(hash_str("colorSeries"), 5372);
      // Java: this.mapobj.put("colorNegative", 5373);
      self.mapobj.insert(hash_str("colorNegative"), 5373);
      // Java: this.mapobj.put("colorAxis", 5374);
      self.mapobj.insert(hash_str("colorAxis"), 5374);
      // Java: this.mapobj.put("colorMarkers", 5375);
      self.mapobj.insert(hash_str("colorMarkers"), 5375);
      // Java: this.mapobj.put("colorFirst", 5376);
      self.mapobj.insert(hash_str("colorFirst"), 5376);
      // Java: this.mapobj.put("colorLast", 5377);
      self.mapobj.insert(hash_str("colorLast"), 5377);
      // Java: this.mapobj.put("colorHigh", 5378);
      self.mapobj.insert(hash_str("colorHigh"), 5378);
      // Java: this.mapobj.put("colorLow", 5379);
      self.mapobj.insert(hash_str("colorLow"), 5379);
      // Java: this.mapobj.put("sparkline", 5380);
      self.mapobj.insert(hash_str("sparkline"), 5380);
      // Java: this.mapobj.put("sparklines", 5381);
      self.mapobj.insert(hash_str("sparklines"), 5381);
      // Java: this.mapobj.put("minAxisType", 5390);
      self.mapobj.insert(hash_str("minAxisType"), 5390);
      // Java: this.mapobj.put("manualMin", 5391);
      self.mapobj.insert(hash_str("manualMin"), 5391);
      // Java: this.mapobj.put("maxAxisType", 5392);
      self.mapobj.insert(hash_str("maxAxisType"), 5392);
      // Java: this.mapobj.put("manualMax", 5393);
      self.mapobj.insert(hash_str("manualMax"), 5393);
      // Java: this.mapobj.put("displayEmptyCellsAs", 5384);
      self.mapobj.insert(hash_str("displayEmptyCellsAs"), 5384);
      // Java: this.mapobj.put("displayXAxis", 5389);
      self.mapobj.insert(hash_str("displayXAxis"), 5389);
      // Java: this.mapobj.put("last", 5387);
      self.mapobj.insert(hash_str("last"), 5387);
      // Java: this.mapobj.put("high", 5385);
      self.mapobj.insert(hash_str("high"), 5385);
      // Java: this.mapobj.put("low", 5386);
      self.mapobj.insert(hash_str("low"), 5386);
      // Java: this.mapobj.put("negative", 5388);
      self.mapobj.insert(hash_str("negative"), 5388);
      // Java: this.mapobj.put("markers", 5394);
      self.mapobj.insert(hash_str("markers"), 5394);
      // Java: this.mapobj.put("displayHidden", 5395);
      self.mapobj.insert(hash_str("displayHidden"), 5395);
      // Java: this.mapobj.put("lineWeight", 5496);
      self.mapobj.insert(hash_str("lineWeight"), 5496);
      // Java: this.mapobj.put("dateAxis", 5497);
      self.mapobj.insert(hash_str("dateAxis"), 5497);
      // Java: this.mapobj.put("axisColor", 5499);
      self.mapobj.insert(hash_str("axisColor"), 5499);
      // Java: this.mapobj.put("allowEditUser", 5500);
      self.mapobj.insert(hash_str("allowEditUser"), 5500);
      // Java: this.mapobj.put("hasInvisiblePropRange", 5501);
      self.mapobj.insert(hash_str("hasInvisiblePropRange"), 5501);
      // Java: this.mapobj.put("rangeList", 5502);
      self.mapobj.insert(hash_str("rangeList"), 5502);
      // Java: this.mapobj.put("sheetStid", 5503);
      self.mapobj.insert(hash_str("sheetStid"), 5503);
      // Java: this.mapobj.put("master", 5504);
      self.mapobj.insert(hash_str("master"), 5504);
      // Java: this.mapobj.put("pwdUUID", 5505);
      self.mapobj.insert(hash_str("pwdUUID"), 5505);
      // Java: this.mapobj.put("arrUserId", 5506);
      self.mapobj.insert(hash_str("arrUserId"), 5506);
      // Java: this.mapobj.put("rangeCreator", 5507);
      self.mapobj.insert(hash_str("rangeCreator"), 5507);
      // Java: this.mapobj.put("othersAccessPermission", 5508);
      self.mapobj.insert(hash_str("othersAccessPermission"), 5508);
      // Java: this.mapobj.put("userID", 5509);
      self.mapobj.insert(hash_str("userID"), 5509);
      // Java: this.mapobj.put("accessPermission", 5510);
      self.mapobj.insert(hash_str("accessPermission"), 5510);
      // Java: this.mapobj.put("woProps", 5511);
      self.mapobj.insert(hash_str("woProps"), 5511);
      // Java: this.mapobj.put("woSheetsProps", 5512);
      self.mapobj.insert(hash_str("woSheetsProps"), 5512);
      // Java: this.mapobj.put("woSheetProps", 5513);
      self.mapobj.insert(hash_str("woSheetProps"), 5513);
      // Java: this.mapobj.put("interlineOnOff", 5514);
      self.mapobj.insert(hash_str("interlineOnOff"), 5514);
      // Java: this.mapobj.put("interlineColor", 5515);
      self.mapobj.insert(hash_str("interlineColor"), 5515);
      // Java: this.mapobj.put("isDbSheet", 5516);
      self.mapobj.insert(hash_str("isDbSheet"), 5516);
      // Java: this.mapobj.put("woBookProps", 5517);
      self.mapobj.insert(hash_str("woBookProps"), 5517);
      // Java: this.mapobj.put("bookSettings", 5518);
      self.mapobj.insert(hash_str("bookSettings"), 5518);
      // Java: this.mapobj.put("isFilterShared", 5519);
      self.mapobj.insert(hash_str("isFilterShared"), 5519);
      // Java: this.mapobj.put("isAutoUpdatePaused", 5520);
      self.mapobj.insert(hash_str("isAutoUpdatePaused"), 5520);
      // Java: this.mapobj.put("filterType", 5521);
      self.mapobj.insert(hash_str("filterType"), 5521);
      // Java: this.mapobj.put("isMergeTasksAutoUpdate", 5522);
      self.mapobj.insert(hash_str("isMergeTasksAutoUpdate"), 5522);
      // Java: this.mapobj.put("dashboard", 5523);
      self.mapobj.insert(hash_str("dashboard"), 5523);
    }

    // Java: SMap()
    fn new_internal() -> Self {
      // Java: super();
      let mut m = SMap {
          mapobj: HashMap::new(),
      };
      // Java: this.init();
      m.init();
      m
    }

    // Java: static SMap getInstance()
    pub fn getInstance() -> &'static SMap {
      // Java: if (obj == null)
      // Java: obj = new SMap();
      // Java: return obj;
      OBJ.get_or_init(|| SMap::new_internal())
    }

    // Java: int getId(int prefix, int name)
    pub fn getId(&self, _prefix: i32, name: i32) -> i32 {
      // Java: return this.mapobj.get(name);
      *self.mapobj.get(&name).unwrap_or(&-1)
    }

}

fn hash_str(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
