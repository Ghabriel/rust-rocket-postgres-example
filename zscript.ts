import { addCommand, exec, run } from '@ghabriel/z-script';

const PROJECT_NAME = 'postgres-test';
const DATABASE_SERVICE_NAME = `${PROJECT_NAME}_db_1`;

addCommand('main', () => {
    exec(`docker-compose -f docker/docker-compose.yml --project-name=${PROJECT_NAME} up`);
});

addCommand('sql', () => {
    exec(
        `docker run --network=${PROJECT_NAME}_default \
	    -it --rm --link ${DATABASE_SERVICE_NAME}:postgres postgres \
	    psql --host postgres --username postgres`
    );
});

run();
