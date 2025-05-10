use yew::prelude::*;

#[function_component]
pub fn Blog() -> Html {
    html! {
        <div class={"flex flex-col gap-4 py-4 px-64 w-screen bg-[url(/static/wallpaper-2.jpg)] bg-repeat-y bg-cover bg-fixed bg-center text-lime-400 font-lg p-4"}>
            <header class={"w-full h-fit p-4 rounded-lg bg-indigo-950/100 backdrop-blur-lg"} >
                <h1 class={"text-cyan-300 text-2xl font-extrabold"}>
                    {"My Month in Tokyo – A Dream in Four Acts"}
                </h1>
                <p class={"text-cyan-300 text-sm"} >
                    {"By Santiago Wu – 22, Paraguayan and Tokyo dreamer."}
                </p>
                <p>
                    {"

こんにちは！ Visiting Tokyo has been a dream of mine for as long as I can remember. I grew up listening to Japanese music, studying it, and learning from Japanese culture. \"I have to visit Tokyo no matter what!\" Then, I'm finally here, at the Narita Airport, asking myself if I am daydreaming or not. Believe it or not, I also have a partner in my travel called Tairyu (太陽竜), a twenty-one years old Japanese student who was in his second year at Keio University. Tairyu is a sunny and polish person. He's extremely extrovert and talky. Even for a Paraguayan, Tairyu is too cheerful."}
                </p>
            </header>
            <div class={"w-full flex flex-row gap-2 rounded-lg"}>
                <div class={"w-[30%] flex justify-center content-center rounded-lg shadow-2xl shadow-cyan-400/50 overflow-clip"} >
                    <img class={"object-cover"} src={"/static/photo-1.png"} />
                </div>
                <div class={"w-full flex flex-col p-4 gap-4 flex-1 rounded-lg bg-indigo-950/100 backdrop-blur-lg shadow-2xl shadow-indigo-950/100 text-amber-400"} >
                    <h2 class={"text-cyan-300 text-xl font-extrabold"}>
                        {"Week 1 – The Dream Begins"}
                    </h2>
                    <p>
                        {"Landing in Tokyo felt like stepping into a scene I had watched a thousand times, but never truly lived. The lights of Shibuya, the smells from the food stalls in Harajuku, the rhythm of the city. It was overwhelming! Asuncion would never have such a rhythm like that.

Our highlight that week was visiting Tokyo Tower. I had seen it countless times in anime and postcards, but nothing compares to standing beneath it, watching the city illuminated by neon lights at the night. I realized that this was just the beginning of an unforgettable experience."}
                    </p>
                </div>
            </div>
            <div class={"w-full flex flex-row gap-2 rounded-lg"}>
                <div class={"w-full flex flex-col p-4 gap-4 flex-1 rounded-lg bg-indigo-950/100 backdrop-blur-lg shadow-2xl shadow-indigo-900/100 text-rose-400"} >
                    <h2 class={"text-cyan-300 text-xl font-extrabold"}>
                        {"Week 2 – Getting Lost on Purpose"}
                    </h2>
                    <p>
                        {"Tairyu had a wild idea: \"Let's get lost. This is not Paraguay, so you can stay quiet.\". From my own experience in Asuncion, getting lost is not something that you want to experiment. But Tokyo is considered among the safest places in the world, so I ended up by accepting his purpose. We chose random train stops and wandered with no plan, and no destination. That's how we found ourselves in Shimokitazawa, sipping tea in a tiny bookshop, chatting in Japanese with the kind owner about literature and Paraguay. He was interested in the almighty tereré and the ka'a he'e (stevia). The man and Tairyu told says that stevia is widely used among Japanese to make sweets, since sugar is perjudicial for health. I don't like stevia too much, but I don't mind consuming it once a while.

Later that week, we run across the best ramen I've ever tasted in a six-seat shop in Ikebukuro. There was nothing more in my mind. Just a steaming bowl of perfection called ramen. In Tokyo, even getting lost leads you to something unforgettable, something I will definitely miss later."}
                    </p>
                </div>
                <div class={"w-[30%] flex justify-center content-center rounded-lg shadow-2xl shadow-cyan-400/50 overflow-clip"} >
                    <img class={"object-cover"} src={"/static/photo-2.png"} />
                </div>
            </div>
            <div class={"w-full flex flex-row gap-2 rounded-lg"}>
                <div class={"w-[30%] flex justify-center content-center rounded-lg shadow-2xl shadow-cyan-400/50 overflow-clip"} >
                    <img class={"object-cover"} src={"/static/photo-1.png"} />
                </div>
                <div class={"w-full flex flex-col p-4 gap-4 flex-1 rounded-lg bg-indigo-950/100 backdrop-blur-lg shadow-2xl shadow-indigo-950/100 text-amber-400"} >
                    <h2 class={"text-cyan-300 text-xl font-extrabold"}>
                        {"Week 3 – Between Tradition and Tomorrow"}
                    </h2>
                    <p>
                        {"This week was a deep dive into contrasts. First, we explored the Sensō-ji Temple in Asakusa, where I pulled a good-luck omikuji and felt a strange sense of peace. Then, we went to Akihabara, where I found myself completely surrounded by neon lights and endless anime merch. I love anime! But Tairyu doesn't. He says that anime is too \"common\". I was depressed by that, but the feeling of time traveling is awesome. Although there are historical landmarks in Asuncion, they feel forgotten by the people. Perhaps we as Paraguayans must revalue our own history an culture. We visited the Odaiba museum, where I saw the life-size Unicorn Gundam statue. At night, it lit up like something out of a sci-fi dream. I realized Tokyo doesn't try to hide its contradictions between the modernized city and the traditional shrines. They celebrate them. They are proud of them."}
                    </p>
                </div>
            </div>
            <div class={"w-full flex flex-row gap-2 rounded-lg"}>
                <div class={"w-full flex flex-col p-4 gap-4 flex-1 rounded-lg bg-indigo-950/100 backdrop-blur-lg shadow-2xl shadow-indigo-900/100 text-rose-400"} >
                    <h2 class={"text-cyan-300 text-xl font-extrabold"}>
                        {"Week 4 – Sayonara, For Now"}
                    </h2>
                    <p>
                        {"This last week was slow, intentional. We revisited our favorite spots, took long walks through Ueno Park, and simply existed in the city. On our final night, we returned to Tokyo Tower, the beginning of my journey, and now its ending.

This time, it felt different. I wasn't just a tourist anymore. I was someone who had memories stitched into the streets, the stations, the language. As I looked out from the tower’s top deck, I made a quiet promise to myself: this is not goodbye. Just a \"see you again.\""}
                    </p>
                </div>
                <div class={"w-[30%] flex justify-center content-center rounded-lg shadow-2xl shadow-cyan-400/50 overflow-clip"} >
                    <img class={"object-cover"} src={"/static/photo-2.png"} />
                </div>
            </div>
            <footer class={"w-full h-fit p-4 rounded-lg bg-indigo-950/100 backdrop-blur-lg"} >
                <h1 class={"text-cyan-300 text-2xl font-extrabold"}>
                    {"Final Thoughts"}
                </h1>
                <p>
                    {"Tokyo didn't just meet my expectations. The were completely surpassed. I came looking for a place I had only dreamed about and now I have to leave this dream to return to my beloved Asuncion. If you ask for Tairyu, we interchanged phone numbers. He says that Japanese people is always kind, but cold and distant on their relationships. But he's an exception to the rule. He says that I'm more Japanese than him. Perhaps, we will meet again in the future. Maybe not. For now,

ありがとう、東京。I'll be back. それは約束だ。"}
                </p>
            </footer>
        </div>
    }
}
