<script>
	let items = [];
	let websocketStatus = "initial";
	let textToAdd;

	// setInterval(() => {
	// 	console.log(document.hasFocus());
	// 	console.log(websocket.readyState);
	// }, 1000);

	// document.addEventListener(
	// 	"onfocus",
	// 	() => {
	// 		console.log("focussed");
	// 	},
	// 	true
	// );

	const websocket = new WebSocket("ws://192.168.1.13:5000/ws");
	websocket.addEventListener("open", () => {
		console.log("websocket connected");
		websocketStatus = "open";
		websocket.send('{"read": {}}');
	});
	websocket.addEventListener("message", ({ data }) => {
		console.log(data);
		items = JSON.parse(data);
	});
	websocket.addEventListener("close", () => {
		websocketStatus = "closed";
	});
</script>

<main>
	<h1>Sybil</h1>

	{#if websocketStatus == "closed"}
		<h2>Websocket disconnected</h2>
	{/if}

	<button
		on:click={() => {
			websocket.send('{"read": {}}');
		}}>update</button
	>

	<label for="text">new note text:</label>
	<input name="text" type="text" bind:value={textToAdd} />

	<button
		on:click={() => {
			websocket.send(JSON.stringify({ create: { text: textToAdd } }));
		}}>add</button
	>

	<table>
		<th>id</th>
		<th>time created</th>
		<th>text</th>

		{#each items as item}
			<tr>
				<td>{item.id}</td>
				<td>{item.time_created}</td>
				<td>{item.text}</td>
			</tr>
			<!-- <p>{item["text"]}</p> -->
		{:else}
			<p>No Results</p>
		{/each}
	</table>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	table,
	th,
	td {
		border: 1px solid black;
		border-collapse: collapse;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
