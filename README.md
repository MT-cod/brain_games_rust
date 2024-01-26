## Учебный проект «Игры разума» в рамках изучения ЯП Rust

<h2>Описание</h2>
<p>«Игры разума» — набор из пяти консольных игр, построенных по принципу популярных мобильных приложений для прокачки мозга. Каждая игра задает вопросы, на которые нужно дать правильные ответы. После трех правильных ответов считается, что игра пройдена. Неправильные ответы завершают игру и предлагают пройти ее заново. Игры:</p>

<ul>
<li>Калькулятор. Арифметические выражения, которые необходимо вычислить.</li>
<li>Прогрессия. Поиск пропущенных чисел в последовательности чисел.</li>
<li>Определение четного числа.</li>
<li>Определение наибольшего общего делителя.</li>
<li>Определение простого числа.</li>
</ul>

<p>Пример игры:</p>
<pre class="hljs"><code class="sh"><span style="color: #008080">$ </span>brain-progression
Welcome to the Brain Game!
What number is missing <span style="color: #000000;font-weight: bold">in </span>this progression?
May I have your name? Roman
Hello, Roman!
Question: 14 .. 18 20 22 24 26 28
Your answer: 16 <span style="color: #999988;font-style: italic"># Пользователь вводит ответ</span>
Correct!
Question: 5 6 7 8 9 .. 11 12
Your answer: 10 <span style="color: #999988;font-style: italic"># Пользователь вводит ответ</span>
Correct!
Question: 12 15 18 21 .. 27 30 33
Your answer: 24 <span style="color: #999988;font-style: italic"># Пользователь вводит ответ</span>
Correct!
Congratulations, Roman!
</code></pre>
