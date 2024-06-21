import React from 'react';
import './App.css';
import mockdatafile from "./mockdata.json";

interface TableEntry {
    "id": string,
    "title": string,
    "channel": string,
    "date": string
}


const columns = [
    { label: "Title", accessor: "title" },
    { label: "Date", accessor: "date" },
    { label: "Channel", accessor: "channel" },
    { label: "ID", accessor: "id" }
];

function Table() {
    function compareEntries(a: TableEntry, b: TableEntry) {
        if (a.title < b.title) {
            return -1;
        } else if (a.title > b.title) {
            return 1;
        }
        return 0;
    }

    let mockData: TableEntry[] = mockdatafile;
    mockData = mockData.sort(compareEntries);

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
