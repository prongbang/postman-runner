use crate::reporter::report::DashboardReport;

pub fn dashboard(report: DashboardReport) -> String {
    let template = r#"<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>Postman Runner Dashboard</title>
    <link href='https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css' rel='stylesheet'>
</head>
<body class='bg-gray-100'>
<div class='container mx-auto p-4'>

    <!-- Header -->
    <div class='lg:flex lg:items-center lg:justify-between mb-10'>
        <div class='min-w-0 flex-1'>
            <h2 class='text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight'>
                <span id='title'>Postman Runner Dashboard</span>
            </h2>
            <div class='mt-1 flex flex-col sm:mt-0 sm:flex-row sm:flex-wrap sm:space-x-6'>
                <div class='mt-2 flex items-center text-sm text-gray-500'>
                    <svg class='mr-1.5 h-5 w-5 flex-shrink-0 text-gray-400' viewBox='0 0 20 20' fill='currentColor'
                         aria-hidden='true'>
                        <path fill-rule='evenodd'
                              d='M5.75 2a.75.75 0 01.75.75V4h7V2.75a.75.75 0 011.5 0V4h.25A2.75 2.75 0 0118 6.75v8.5A2.75 2.75 0 0115.25 18H4.75A2.75 2.75 0 012 15.25v-8.5A2.75 2.75 0 014.75 4H5V2.75A.75.75 0 015.75 2zm-1 5.5c-.69 0-1.25.56-1.25 1.25v6.5c0 .69.56 1.25 1.25 1.25h10.5c.69 0 1.25-.56 1.25-1.25v-6.5c0-.69-.56-1.25-1.25-1.25H4.75z'
                              clip-rule='evenodd'/>
                    </svg>
                    <span id='createdAt'></span>
                </div>
            </div>
        </div>
    </div>

    <!-- Card Summary -->
    <div class='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-10'>
        <div class='bg-white p-6 rounded-lg shadow-md'>
            <h2 class='text-lg font-semibold mb-2 upper'>TOTAL ITERATIONS</h2>
            <p id='totalIterations' class='text-6xl font-semibold'>0</p>
        </div>
        <div class='bg-white p-6 rounded-lg shadow-md'>
            <h2 class='text-lg font-semibold mb-2'>TOTAL ASSERTIONS</h2>
            <p id='totalAssertions' class='text-6xl font-semibold'>0</p>
        </div>
        <div class='bg-white p-6 rounded-lg shadow-md'>
            <h2 class='text-lg font-semibold mb-2'>TOTAL FAILED TESTS</h2>
            <p id='totalFailedTests' class='text-6xl font-semibold'>0</p>
        </div>
        <div class='bg-white p-6 rounded-lg shadow-md'>
            <h2 class='text-lg font-semibold mb-2'>TOTAL SKIPPED TESTS</h2>
            <p id='totalSkippedTests' class='text-6xl font-semibold'>0</p>
        </div>
    </div>

    <!-- Dashboard Cards -->
    <h1 class='text-2xl font-bold mb-6'>2 Collections</h1>

    <div class='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4' id='collections'></div>
</div>
<script>
    // Title
    document.title = '{{title}}';
    document.querySelector('#title').innerHTML = '{{title}}';

    // Created At
    document.querySelector('#createdAt').innerHTML = '{{createdAt}}';

    // Summary
    document.querySelector('#totalIterations').innerHTML = '{{totalIterations}}';
    document.querySelector('#totalAssertions').innerHTML = '{{totalAssertions}}';
    document.querySelector('#totalFailedTests').innerHTML = '{{totalFailedTests}}';
    document.querySelector('#totalSkippedTests').innerHTML = '{{totalSkippedTests}}';

    // Collections
    let collections = {{collections}};

    const collectionsElement = document.getElementById('collections');
    collections.forEach((value) => {
        const card = document.createElement('div');
        card.innerHTML = `
             <a href='${value.reportUrl}'>
                <div class='bg-white p-4 rounded-lg shadow-md'>
                    <h2 class='text-lg font-semibold mb-4'>${value.collectionName}</h2>
                    <div class='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4'>
                        <div>
                            <div class='text-xs mb-3'>Interactions</div>
                            <p class='text-2xl'>${value.interactions}</p>
                        </div>
                        <div>
                            <div class='text-xs mb-3'>Assertions</div>
                            <p class='text-2xl'>${value.assertions}</p>
                        </div>
                        <div>
                            <div class='text-xs mb-3'>Failed</div>
                            <p class='text-2xl'>${value.failed}</p>
                        </div>
                        <div>
                            <div class='text-xs mb-3'>Skipped</div>
                            <p class='text-2xl'>${value.skipped}</p>
                        </div>
                    </div>
                </div>
            </a>`;
        collectionsElement.appendChild(card);
    });
</script>
</body>
</html>"#;



    let json_collections = serde_json::to_string(&report.collections).unwrap();
    format!(
        "{}",
        template.replace("{{title}}", report.title.as_str())
            .replace("{{createdAt}}", report.created_at.as_str())
            .replace("{{totalIterations}}", &format!("{}", report.total_iterations))
            .replace("{{totalAssertions}}", &format!("{}", report.total_assertions))
            .replace("{{totalFailedTests}}", &format!("{}", report.total_failed_tests))
            .replace("{{totalSkippedTests}}", &format!("{}", report.total_skipped_tests))
            .replace("{{collections}}", json_collections.as_str())
    )
}