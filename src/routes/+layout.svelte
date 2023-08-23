<script>
	import '../app.css';
</script>

<div id="pg_body" class="flex flex-col w-screen h-screen border relative max-w-[calc(100vw-18px)]">
	<!--
	<div class="flex items-center justify-end px-2 w-full h-11 bg-base-blue border" />
-->
	<div class="flex items-start gap-x-2 bg-[#FAFAFA] border w-full min-h-[644px] h-full p-1">
		<slot />
	</div>
</div>
