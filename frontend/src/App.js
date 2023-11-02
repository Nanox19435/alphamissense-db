import React, { useState } from 'react';
import './App.css';
import Toolbar from './Toolbar';
import SearchBar from './SearchBar';
import VariantInput from './VariantInput';

const App = () => {
  // La idea es que mostraremos la barra de búsqueda si el estado es la cadena vacía.
  const [geneId, setGeneId] = useState('')

  const setId = (id) => {
    setGeneId(id);
  };

  return (
    <div className="app">
      {geneId === '' ? (
        <div>
          <Toolbar />
          <SearchBar setGeneId={setGeneId}/>
        </div>
      )
        : (
          <div>
            <VariantInput id={geneId}/>
          </div>
        )}
    </div>
  );
};

export default App;