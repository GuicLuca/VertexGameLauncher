interface Game {
    id: number;
    title: string;
    subtitle: string;
    description: string;
    background_image: {
        url: string;
        name: string;
        local_path: string;
        revision: number;
    };
    navigation_icon: {
        url: string;
        name: string;
        local_path: string;
        revision: number;
    };
    game_archive: {
        link: {
            url: string;
            name: string;
            local_path: string;
            revision: number;
        }
        need_extract: boolean;
        strip_top_level_folder: boolean;
        path_to_executable: string;
    };
    version: string;
    platform: string[];
    tags: string[];
}

export default Game;