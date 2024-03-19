--
-- PostgreSQL database dump
--

-- Dumped from database version 15.4 (Debian 15.4-1.pgdg120+1)
-- Dumped by pg_dump version 15.4 (Debian 15.4-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: items; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.items (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    list_id uuid,
    name character varying(255) NOT NULL,
    done boolean DEFAULT false NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    lat double precision,
    lng double precision
);


ALTER TABLE public.items OWNER TO admin;

--
-- Name: lists; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.lists (
    id uuid NOT NULL,
    title character varying(255) NOT NULL,
    user_id uuid NOT NULL
);


ALTER TABLE public.lists OWNER TO admin;

--
-- Name: users; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255) NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    roles character varying NOT NULL
);


ALTER TABLE public.users OWNER TO admin;

--
-- Data for Name: items; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.items (id, list_id, name, done, deleted, lat, lng) FROM stdin;
e2ba29ea-d2f1-4944-b2ad-a5690525ecea	b17f884b-dd39-40f3-907e-bc7b274c1f40	Milk	f	f	\N	\N
7d30141d-120e-44a0-a43b-356847d46adc	509eec5e-df3d-4f27-b0aa-54df81aa2381	Steak	t	f	55.3530621	10.3442499
e1b34b48-7a74-481e-a8e8-159c07916b93	509eec5e-df3d-4f27-b0aa-54df81aa2381	Chicken	t	f	55.3530621	10.3442499
d0117d00-d160-4da7-b9cc-582179cf8167	509eec5e-df3d-4f27-b0aa-54df81aa2381	Pasta	t	f	55.3530628	10.3442416
c6cd99d8-800f-4c06-a1ab-f10d92279e3f	509eec5e-df3d-4f27-b0aa-54df81aa2381	Rice	t	f	55.3530628	10.3442416
f3afd508-bfad-4f51-b8d1-ec600820c0a7	509eec5e-df3d-4f27-b0aa-54df81aa2381	Cheese	t	f	55.3530717	10.3442589
7d565bc9-ce61-4947-8846-bcd79b9736b5	509eec5e-df3d-4f27-b0aa-54df81aa2381	Milk	t	f	55.3530717	10.3442589
3bc37035-ee30-4764-89d4-7fe1ee539589	509eec5e-df3d-4f27-b0aa-54df81aa2381	Cola	t	f	55.3531045	10.3443968
\.


--
-- Data for Name: lists; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.lists (id, title, user_id) FROM stdin;
509eec5e-df3d-4f27-b0aa-54df81aa2381	Shopping	c54a5ed4-d788-4a51-a18b-778f0b2d61bc
509eec5e-df3d-4f27-b0aa-54df81aa2381	Shopping	2e86e5e4-530e-40c1-905d-6c45bf38feda
b17f884b-dd39-40f3-907e-bc7b274c1f40	Personal	c54a5ed4-d788-4a51-a18b-778f0b2d61bc
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.users (id, first_name, last_name, username, password, roles) FROM stdin;
c54a5ed4-d788-4a51-a18b-778f0b2d61bc	Anders	Madsen	anderslm@hotmail.com	password	admin,user
2e86e5e4-530e-40c1-905d-6c45bf38feda	Nadia	Thomsen	nadiasophie@hotmail.com	password	admin,user
\.


--
-- Name: items items_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT items_pkey PRIMARY KEY (id);


--
-- Name: lists lists_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.lists
    ADD CONSTRAINT lists_pkey PRIMARY KEY (id, user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: lists lists_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.lists
    ADD CONSTRAINT lists_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


-- ALTER TABLE items ADD COLUMN geom geometry(Point, 2950);
-- UPDATE items SET geom = ST_SetSRID(ST_MakePoint(lat, lng), 2950);
-- CREATE INDEX ON items USING GIST(geom);
-- UPDATE items SET category = subquery.cid FROM (SELECT id, ST_ClusterDBSCAN(geom, eps := 0.002/6371, minPoints := 1) OVER() AS cid FROM items) AS subquery WHERE items.id = subquery.id;


--
-- PostgreSQL database dump complete
--
