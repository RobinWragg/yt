import React from 'react';
import { useState } from "react";
import './App.css';
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
    { label: "Channel", accessor: "channel" },
    { label: "Title", accessor: "title" },
    { label: "Date", accessor: "date" },
];

function Table() {
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
        window.open("http://" + entryId);
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

    const thStyle: CSS.Properties = {
        cursor: "pointer",
        userSelect: "none",
    };

    return <table>
        <thead>
            <tr>
                {columns.map(({ label, accessor }) => {
                    return <th onClick={() => onHeaderClick(accessor, reverseSortOrder)} style={thStyle} key={accessor}>{label}</th>;
                })}
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            {mockData.map((entry) => {
                return (
                    <tr key={entry.id}>
                        {columns.map(({ accessor }) => {
                            return <td key={accessor}>{entry[accessor as keyof TableEntry]}</td>;
                        })}
                        <td>
                            <button title="Watch" onClick={() => watchEntry(entry.id)}>👁️</button>
                            &nbsp;
                            <button title="Delete" onClick={() => deleteEntry(entry.id)}>❌</button>
                        </td>
                    </tr>
                );
            })}
        </tbody>
    </table >
}

function App() {
    return (
        <Table />
    );
}

export default App;
