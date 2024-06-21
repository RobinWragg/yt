import React from 'react';
import './App.css';
import mockdatafile from "./mockdata.json";

interface TableEntry {
    "id": string,
    "title": string,
    "channel": string,
    "date": string
}

const mockData: TableEntry[] = mockdatafile;

const columns = [
    { label: "Title", accessor: "title" },
    { label: "Date", accessor: "date" },
    { label: "Channel", accessor: "channel" },
    { label: "ID", accessor: "id" }
];

function Table() {
    return <table>
        <thead>
            <tr>
                {columns.map(({ label, accessor }) => {
                    return <th key={accessor}>{label}</th>;
                })}
            </tr>
        </thead>
        <tbody>
            {mockData.map((entry) => {
                return (
                    <tr key={entry.id}>
                        {columns.map(({ accessor }) => {
                            const tEntry = entry[accessor as keyof TableEntry] ? entry[accessor as keyof TableEntry] : "?";
                            return <td key={accessor}>{tEntry}</td>;
                        })}
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
