import { useState } from "react";
import mockdatafile from "./mockdata.json";
import CSS from "csstype";

interface TableEntry {
    "id": string,
    "title": string,
    "channel": string,
    "date": string
}

const columns = [
    { label: "ID", accessor: "id" },
    { label: "Date", accessor: "date" },
    { label: "Channel", accessor: "channel" },
    { label: "Title", accessor: "title" },
];

export default function Home() {
    const [sortMode, setSortMode] = useState("date");
    const [reverseSortOrder, setReverseSortOrder] = useState<boolean>(false);

    function onHeaderClick(accessor: string, previousSortOrder: boolean) {
        if (sortMode === accessor) {
            setReverseSortOrder(!previousSortOrder);
        } else {
            setSortMode(accessor);
            setReverseSortOrder(false);
        }
    }

    function deleteEntry(entryId: string) {
        console.log("delete", entryId);
        // TODO: I need to re-render now but not sure of the non-discouraged way to do it.
    }

    function watchEntry(entryId: string) {
        window.open("https://www.youtube.com/watch?v=" + entryId);
        deleteEntry(entryId);
    }

    let mockData: TableEntry[] = mockdatafile;

    if (sortMode !== "") {
        mockData = mockData.sort((a: TableEntry, b: TableEntry) => {
            if (reverseSortOrder) {
                [a, b] = [b, a];
            }
            return a[sortMode as keyof TableEntry].localeCompare(b[sortMode as keyof TableEntry]);
        });
    }

    // TODO: center table

    const thStyle: CSS.Properties = {
        cursor: "pointer",
        userSelect: "none",
    };

    function trStyle(rowIndex: number) {
        return {
            backgroundColor: rowIndex % 2 === 0 ? "#444" : "black",
        };
    }

    // borderCollapse is needed to color rows correctly.
    return <table style={{ borderCollapse: "collapse", margin: "auto" }}>
        <thead>
            <tr>
                {columns.map(({ label, accessor }) => {
                    return <th onClick={() => onHeaderClick(accessor, reverseSortOrder)} style={thStyle} key={accessor}>{label}</th>;
                })}
            </tr>
        </thead>
        <tbody>
            {mockData.map((entry, rowIndex) => {
                return (
                    <tr key={entry.id} style={trStyle(rowIndex)}>
                        {columns.map(({ accessor }) => {
                            return <td key={accessor}>{entry[accessor as keyof TableEntry]}</td>;
                        })}
                        <td>
                            <button title="Watch" onClick={() => watchEntry(entry.id)}>👁️</button>
                            <button title="Delete" onClick={() => deleteEntry(entry.id)}>❌</button>
                        </td>
                    </tr>
                );
            })}
        </tbody>
    </table >
}
