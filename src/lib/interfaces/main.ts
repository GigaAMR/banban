export interface Activity {
    name: string,
    body?: string
    tags: Array<number>
    ord: number
}

export type Activities = Map<number, Activity>

export interface Column {
    name: string,
    activities: Map<number, Activity>,
    ord: number
}

export type Columns = Map<number, Column>

export interface Tag {
    name: string;
    color?: string
    ord: number
}

export type Tags = Map<number, Tag>


export interface Category {
    name: string,
    tags: Array<number>,
    ord: number
}

export type Categories = Map<number, Category>


export interface Editable {
    id: number;
    field: ActiveField
}

export enum ActiveField {
    ActivityName = "activity name",
    ActivityBody = "activity body",
    ColumnName = "column name"
}

export enum DrawerTab {
    Activity = "activity",
    Settings = "settings"
}