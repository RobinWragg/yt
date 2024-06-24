import React from 'react';
import { useState } from "react";
import './App.css';
import mockdatafile from "./mockdata.json";

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
    const [sortMode, setSortMode] = useState("");

    function compareEntries(a: TableEntry, b: TableEntry) {
        return a[sortMode as keyof TableEntry].localeCompare(b[sortMode as keyof TableEntry]);
    }

    let mockData: TableEntry[] = mockdatafile;

    if (sortMode !== "") {
        mockData = mockData.sort(compareEntries);
    }

    return <table>
        <thead>
            <tr>
                {columns.map(({ label, accessor }) => {
                    return <th onClick={() => setSortMode(accessor)} key={accessor}>{label}</th>;
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
                        <td><button title="Watch">👁️</button> <button title="Delete">❌</button></td>
                    </tr>
                );
            })}
        </tbody>
    </table>
}

function App() {
    return (
        <Table />
    );
}

export default App;
