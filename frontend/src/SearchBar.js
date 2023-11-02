import React, { useRef, useState } from 'react';
import './SearchBar.css'; // Import CSS file for SearchBar component

const SearchBar = ({setGeneId}) => {
  const inputRef = useRef(null);
  const [searchResults, setSearchResults] = useState([]);

  const handleSearch = async () => {
    const searchText = inputRef.current.value;

    try {
      const response = await fetch(`http://127.0.0.1:8000/search/${searchText}`);
      if (response.ok) {
        const result = await response.json();
        console.log("Recieved: ", result);
        setSearchResults(result); // Set the fetched array to state
      } else {
        console.error('Error fetching search data:', response.status);
      }
    } catch (error) {
      console.error('Error fetching search data:', error);
    }
  };

  const handleItemClick = (item) => {
    setGeneId(item);
  };

  return (
    <div className="search-bar">
      <div>
        <input type="text" placeholder="Introduzca aquí el nombre del Gen" ref={inputRef} />
        <button onClick={handleSearch}>Buscar</button>
      </div>
      <div className="item-list">
        {searchResults.map((pair, index) => (
          <button key={index} onClick={() => handleItemClick(pair[0])}>
            {pair[1]}
          </button>
        ))}
      </div>
    </div>
  );
};

export default SearchBar;