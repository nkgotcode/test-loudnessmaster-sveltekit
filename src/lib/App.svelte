<!-- App.svelte -->
<script>
  import { onDestroy, onMount } from "svelte";
  import Result from "./components/Result.svelte";
  import { fly, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { dndzone, SHADOW_ITEM_MARKER_PROPERTY_NAME } from "svelte-dnd-action";
  import NavBar from "./components/NavBar.svelte";
  import Analyzer from "./components/Analyzer.svelte";
  import { loaded, results } from "./components/store";
  import { cubicIn } from "svelte/easing";

  let worker;
  let audioPlayer;

  function handleFileNameUpdate(event) {
    const { fileName, result } = event.detail;
    let currentResults = $results;
    let newResults = [
      ...currentResults,
      {
        id: currentResults.length,
        fileName,
        result,
        isProcessed: true,
      },
    ];

    // Insert placeholders to position Analyzer in the center
    // const totalItems = newResults.length + 1; // +1 for the Analyzer
    // const nextPosition = totalItems % 4; // 4 columns grid
    // let placeholdersToAdd = 0;

    // if (nextPosition === 1)
    //   placeholdersToAdd = 2; // Position 1: 2 placeholders to push Analyzer to center
    // else if (nextPosition === 2) placeholdersToAdd = 1; // Position 2: 1 placeholder to push Analyzer to center

    // for (let i = 0; i < placeholdersToAdd; i++) {
    //   newResults.push({
    //     id: -1,
    //     isProcessed: false,
    //     result: "placeholder",
    //   });
    // }

    results.set(newResults);
    console.log($results);
    scrollToBottom();
  }

  function handleReorder(event) {
    results.set(event.detail.items);
  }
  let dropTargetStyle;
  function handleDndConsider(e) {
    results.set(e.detail.items);
    dropTargetStyle = {};
  }
  function handleDndFinalize(e) {
    results.set(e.detail.items);
    dropTargetStyle = {};
  }
  function handleResultDropped(event) {
    const { source, target } = event.detail;
    // Clone the current results array for immutability
    let updatedResults = [...$results];

    // Find the indices of the source and target in the results array
    const sourceIndex = updatedResults.findIndex(
      (item) => item.id === source.id
    );
    const targetIndex = updatedResults.findIndex(
      (item) => item.id === target.id
    );

    // Swap the source and target in the array
    if (sourceIndex > -1 && targetIndex > -1) {
      updatedResults[sourceIndex] = target;
      updatedResults[targetIndex] = source;
    }

    // Update the results store with the rearranged array
    results.set(updatedResults);
  }

  function scrollToBottom() {
    const scrollHeight = document.documentElement.scrollHeight;
    const position = scrollHeight - window.innerHeight;
    window.scrollTo({ top: position, behavior: "smooth" });
  }

  onMount(async () => {
    worker = new Worker("src/components/audioProcessor.worker.js");
    loaded.set(true);
    audioPlayer = document.getElementById("audioPlayer");
  });

  onDestroy(() => {
    worker?.terminate();
  });
</script>

<NavBar />
<main>
  <!--       use:dndzone={{
        items: $results,
        flipDurationMs: 300,
        dropTargetStyle: dropTargetStyle,
      }} 
          on:consider={handleDndConsider}
      on:finalize={handleDndFinalize}-->
  {#if $loaded}
    <div class="analyzers-container">
      {#each $results.filter((item) => item.isProcessed) as resultData, index (resultData.id)}
        <div class="grid" animate:flip={{ duration: 400 }}>
          <Result data={resultData} on:resultDropped={handleResultDropped} />
          <!-- {#if resultData[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
            <div class="custom-shadow-item">
              {resultData.fileName}
            </div>
          {/if} -->
        </div>
      {/each}
      <div class="analyzer">
        <Analyzer {worker} on:analyzerFinished={handleFileNameUpdate} />
      </div>
    </div>
  {/if}

  <script src="aurora.js"></script>
  <script src="flac.js"></script>
  <script src="alac.js"></script>
  <script src="aac.js"></script>
  <script src="mp3.js"></script>
  <!-- <script src="ia_mpeghd_testbench.js"></script> -->
</main>

<style>
  main {
    display: block;
    box-sizing: border-box;
    padding-top: 64px;
    width: 100%;
    height: 100%;
    background-color: #0f0f0f;
  }
  .analyzers-container > div {
    transition: transform 1000ms;
  }
  .analyzers-container {
    display: grid;
    grid-template-columns: repeat(
      4,
      1fr
    ); /* This will divide the container into 3 equal columns */
    grid-auto-flow: row;
    gap: 20px;
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    padding-top: 20px;
  }
  .analyzer {
    padding-top: 20px;
  }
  .custom-shadow-item {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    visibility: visible;
    border: 1px solid grey;
    /* background: #fefefe; */
    opacity: 0.4;
    margin: 0;
    /* transition: background-color 1000ms ease-in; */
  }
  /* Responsive for mobile devices */

  @media (max-width: 1950px) {
    .analyzers-container {
      grid-template-columns: repeat(
        3,
        1fr
      ); /* 2 items in a row on tablet screens */
      transition: all 1s ease-in-out;
    }
  }

  @media (max-width: 1350px) {
    .analyzers-container {
      grid-template-columns: repeat(
        2,
        1fr
      ); /* 2 items in a row on tablet screens */
      transition: all 1s ease-in-out;
    }
  }

  @media (max-width: 880px) {
    .analyzers-container {
      grid-template-columns: 1fr; /* 1 item in a row on mobile screens */
      transition: all 1s ease-in-out;
    }
  }
</style>
